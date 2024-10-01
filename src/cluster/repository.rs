use async_trait::async_trait;
use sea_orm::IntoActiveModel;
use std::{collections::HashMap, error::Error, sync::Arc};
use tokio::sync::Mutex;

use sea_orm::{prelude::Uuid, ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::{
    cache::{cache_keys::CacheKeys, Cache},
    database::Database,
    entities::{
        clusters::{self, Entity as Cluster},
        namespaces::{self, Entity as Namespace},
    },
    repositories::cluster_repository::ClusterRepository,
};

pub struct ClusterRepositorySea {
    db: Arc<Database>,
    cache: Arc<Mutex<dyn Cache + Send + Sync>>,
}

impl ClusterRepositorySea {
    pub fn new(db: Arc<Database>, cache: Arc<Mutex<dyn Cache + Send + Sync>>) -> Self {
        ClusterRepositorySea { db, cache }
    }
}

#[async_trait]

impl ClusterRepository for ClusterRepositorySea {
    async fn m_get(
        &self,
        cluster_ids: Vec<Uuid>,
    ) -> Result<Vec<clusters::Model>, Box<dyn Error + Send + Sync>> {
        let mut cache = self.cache.lock().await;
        let cached_data: Vec<serde_json::Value> = match cache.m_get(
            cluster_ids
                .iter()
                .map(|id| CacheKeys::Cluster(id.to_string()).key())
                .collect::<Vec<String>>(),
        ) {
            Ok(data) => data,
            Err(_) => Vec::new(),
        };
        let mut clusters: Vec<clusters::Model> = Vec::new();
        for data in cached_data.iter() {
            let cluster: clusters::Model = serde_json::from_value(data.clone())?;
            clusters.push(cluster);
        }
        if clusters.len() == cluster_ids.len() {
            return Ok(clusters);
        }
        let mut query = Cluster::find();
        query = query.filter(clusters::Column::Id.is_in(cluster_ids));
        clusters = query.all(&self.db.connection).await?;
        let mut redis_items: HashMap<String, String> = HashMap::new();
        for cluster in clusters.iter() {
            let value = serde_json::to_string(&cluster)?;
            let key = CacheKeys::Cluster(cluster.id.to_string()).key();
            redis_items.insert(key, value);
        }
        cache.m_save(redis_items)?;
        Ok(clusters)
    }

    async fn get_namespaces(
        &self,
        cluster_id: Uuid,
    ) -> Result<Vec<namespaces::Model>, Box<dyn Error + Send + Sync>> {
        let namespaces = Namespace::find()
            .filter(namespaces::Column::ClusterId.eq(cluster_id))
            .all(&self.db.connection)
            .await?;
        let mut cached_namespaces: HashMap<String, String> = HashMap::new();
        for namespace in namespaces.iter() {
            let cache_key = CacheKeys::Namespace(namespace.id.to_string()).key();
            let value = serde_json::to_string(&namespace)?;
            cached_namespaces.insert(cache_key, value);
        }
        let mut cache = self.cache.lock().await;
        let _ = cache.m_save(cached_namespaces).map_err(|_| {});
        Ok(namespaces)
    }

    async fn get_all(&self) -> Result<Vec<clusters::Model>, Box<dyn Error + Send + Sync>> {
        let mut cache = self.cache.lock().await;
        let clusters = Cluster::find().all(&self.db.connection).await?;
        let mut redis_items: HashMap<String, String> = HashMap::new();
        for cluster in clusters.iter() {
            let value = serde_json::to_string(&cluster)?;
            let key = CacheKeys::Cluster(cluster.id.to_string()).key();
            redis_items.insert(key, value);
        }
        // Caching should not be a blocking operation. Therefore, we ignore the result
        // I am convinced that there are better ways to handle this
        let _ = cache.m_save(redis_items).map_err(|_| {});
        Ok(clusters)
    }

    async fn get(&self, id: Uuid) -> Result<clusters::Model, Box<dyn Error + Send + Sync>> {
        let mut cache = self.cache.lock().await;
        let cache_key = CacheKeys::Cluster(id.to_string()).key();
        match cache.get(cache_key.as_str()) {
            Ok(value) => {
                let cluster: clusters::Model = serde_json::from_value(value)?;
                return Ok(cluster);
            }
            _ => {}
        };
        let res = match Cluster::find_by_id(id).one(&self.db.connection).await {
            Ok(cluster) => cluster,
            Err(e) => return Err(Box::new(e)),
        };
        let cluster = match res {
            Some(cluster) => cluster,
            None => {
                return Err(Box::new(DbErr::RecordNotFound(format!(
                    "Cluster with id {} not found",
                    id,
                ))))
            }
        };
        let value = serde_json::to_string(&cluster)?;
        let _ = cache.save(&cache_key, value).map_err(|_| {});
        Ok(cluster)
    }

    async fn create(
        &self,
        cluster: clusters::Model,
    ) -> Result<clusters::Model, Box<dyn Error + Send + Sync>> {
        let cluster: clusters::Model = Cluster::insert(cluster.into_active_model())
            .exec_with_returning(&self.db.connection)
            .await?;
        self.cache.lock().await.save(
            CacheKeys::Cluster(cluster.id.to_string()).key().as_str(),
            serde_json::to_string(&cluster)?,
        )?;
        Ok(cluster)
    }

    async fn update(
        &self,
        cluster: clusters::Model,
    ) -> Result<clusters::Model, Box<dyn Error + Send + Sync>> {
        let cluster = cluster
            .into_active_model()
            .update(&self.db.connection)
            .await?;
        let cache_key = CacheKeys::Cluster(cluster.id.to_string()).key();
        let mut cache = self.cache.lock().await;
        let _ = cache
            .save(cache_key.as_str(), serde_json::to_string(&cluster)?)
            .map_err(|_| {});
        Ok(cluster)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        Cluster::delete_by_id(id).exec(&self.db.connection).await?;
        let cache_key = CacheKeys::Cluster(id.to_string()).key();
        let mut cache = self.cache.lock().await;
        let _ = cache.invalidate(cache_key.as_str()).map_err(|_| {});
        Ok(())
    }
}
