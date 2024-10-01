use std::{collections::HashMap, error::Error, sync::Arc};

use async_trait::async_trait;
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::{
    cache::{cache_keys::CacheKeys, Cache},
    database::Database,
    entities::namespaces,
    entities::namespaces::Entity as Namespace,
    repositories::namespace_repository::NamespaceRepository,
};

pub struct NamespaceRepositoryImpl {
    pub db: Arc<Database>,
    pub cache: Arc<Mutex<dyn Cache + Send + Sync>>,
}

impl NamespaceRepositoryImpl {
    pub fn new(db: Arc<Database>, cache: Arc<Mutex<dyn Cache + Send + Sync>>) -> Self {
        NamespaceRepositoryImpl { db, cache }
    }
}

#[async_trait]
impl NamespaceRepository for NamespaceRepositoryImpl {
    async fn get(&self, id: Uuid) -> Result<namespaces::Model, Box<dyn Error + Send + Sync>> {
        let mut cache = self.cache.lock().await;
        let cached_data: Value = match cache.get(&CacheKeys::Namespace(id.to_string()).key()) {
            Ok(data) => data,
            _ => Value::Null,
        };
        match cached_data {
            Value::Null => {}
            _ => {
                let namespace: namespaces::Model = serde_json::from_value(cached_data)?;
                return Ok(namespace);
            }
        }
        let res = Namespace::find_by_id(id).one(&self.db.connection).await?;
        let namespace = match res {
            Some(namespace) => namespace,
            None => return Err(Box::from("Namespace not found")),
        };
        let value = serde_json::to_string(&namespace)?;
        cache.save(&CacheKeys::Namespace(id.to_string()).key(), value)?;
        Ok(namespace)
    }
    async fn mget(
        &self,
        ids: Vec<Uuid>,
    ) -> Result<Vec<namespaces::Model>, Box<dyn Error + Send + Sync>> {
        let mut cache = self.cache.lock().await;
        let cached_data: Vec<Value> = match cache.m_get(
            ids.iter()
                .map(|id| CacheKeys::Namespace(id.to_string()).key())
                .collect::<Vec<String>>(),
        ) {
            Ok(data) => data,
            _ => Vec::new(),
        };
        let mut namespaces: Vec<namespaces::Model> = Vec::new();
        for data in cached_data.iter() {
            let namespace: namespaces::Model = serde_json::from_value(data.clone())?;
            namespaces.push(namespace);
        }
        if namespaces.len() == ids.len() {
            return Ok(namespaces);
        }
        let mut query = Namespace::find();
        query = query.filter(namespaces::Column::Id.is_in(ids));
        namespaces = query.all(&self.db.connection).await?;
        let mut redis_items: HashMap<String, String> = HashMap::new();
        for namespace in namespaces.iter() {
            let value = serde_json::to_string(&namespace)?;
            let key = CacheKeys::Namespace(namespace.id.to_string()).key();
            redis_items.insert(key, value);
        }
        cache.m_save(redis_items)?;
        Ok(namespaces)
    }
    async fn get_all(&self) -> Result<Vec<namespaces::Model>, Box<dyn Error + Send + Sync>> {
        let mut cache = self.cache.lock().await;
        let namespaces = Namespace::find().all(&self.db.connection).await?;
        let mut redis_items: HashMap<String, String> = HashMap::new();
        for namespace in namespaces.iter() {
            let value = serde_json::to_string(&namespace)?;
            let key = CacheKeys::Namespace(namespace.id.to_string()).key();
            redis_items.insert(key, value);
        }
        cache.m_save(redis_items).map_err(|_| {});
        Ok(namespaces)
    }
    async fn create(
        &self,
        namespace: namespaces::Model,
    ) -> Result<namespaces::Model, Box<dyn Error + Send + Sync>> {
        let namespace = namespace
            .into_active_model()
            .insert(&self.db.connection)
            .await?;
        let value = serde_json::to_string(&namespace)?;
        let key = CacheKeys::Namespace(namespace.id.to_string()).key();
        let mut cache = self.cache.lock().await;
        cache.save(&key, value)?;
        Ok(namespace)
    }
    async fn update(
        &self,
        namespace: namespaces::Model,
    ) -> Result<namespaces::Model, Box<dyn Error + Send + Sync>> {
        let namespace = namespace
            .into_active_model()
            .update(&self.db.connection)
            .await?;
        let value = serde_json::to_string(&namespace)?;
        let key = CacheKeys::Namespace(namespace.id.to_string()).key();
        let mut cache = self.cache.lock().await;
        cache.save(&key, value)?;
        Ok(namespace)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        let key = CacheKeys::Namespace(id.to_string()).key();
        let mut cache = self.cache.lock().await;
        cache.invalidate(&key)?;
        Namespace::delete_by_id(id)
            .exec(&self.db.connection)
            .await
            .map_err(|_| "Failed to delete namespace")?;
        Ok(())
    }
}
