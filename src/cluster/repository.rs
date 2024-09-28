use std::{
    error::Error,
    sync::{Arc, RwLock},
};

use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::{
    cache::Cache,
    database::Database,
    entities::{
        clusters::{self, Entity as Cluster},
        namespaces::{self, Entity as Namespace},
    },
    repositories::cluster_repository::ClusterRepository,
};

pub struct ClusterRepositorySea {
    db: Arc<Database>,
    cache: Arc<RwLock<Cache>>,
    cache_key: String,
}

impl ClusterRepositorySea {
    pub fn new(db: Arc<Database>, cache: Arc<RwLock<Cache>>) -> Self {
        ClusterRepositorySea {
            db,
            cache,
            cache_key: "cluster:".to_string(),
        }
    }
}
// TODO: Implement caching
impl ClusterRepository for ClusterRepositorySea {
    async fn m_get(&self, cluster_ids: Vec<Uuid>) -> Result<Vec<clusters::Model>, Box<dyn Error>> {
        // TODO: Create two list on for failure and one for success
        let mut query = Cluster::find();
        query = query.filter(clusters::Column::Id.is_in(cluster_ids));
        let clusters = query.all(&self.db.connection).await?;
        Ok(clusters)
    }

    async fn get_namespaces(
        &self,
        cluster_id: Uuid,
    ) -> Result<Vec<namespaces::Model>, Box<dyn Error>> {
        // TODO: verify that cluster exists
        self.get(cluster_id).await?;
        let namespaces = Namespace::find()
            .filter(namespaces::Column::ClusterId.eq(cluster_id))
            .all(&self.db.connection)
            .await?;
        Ok(namespaces)
    }

    async fn get_all(&self) -> Result<Vec<clusters::Model>, Box<dyn Error>> {
        let clusters = Cluster::find().all(&self.db.connection).await?;
        Ok(clusters)
    }

    async fn get(&self, id: Uuid) -> Result<clusters::Model, Box<dyn Error>> {
        let res = Cluster::find_by_id(id).one(&self.db.connection).await?;

        let cluster = match res {
            Some(cluster) => cluster,
            None => {
                return Err(Box::new(DbErr::RecordNotFound(format!(
                    "Cluster with id {} not found",
                    id,
                ))))
            }
        };
        let mut cache = self.cache.write().unwrap();
        let value = serde_json::json!(cluster);
        cache.save(&format!("{}{}", self.cache_key, id), value)?;
        Ok(cluster)
    }

    async fn create(&self, cluster: clusters::Model) -> Result<clusters::Model, Box<dyn Error>> {
        let cluster = cluster
            .into_active_model()
            .insert(&self.db.connection)
            .await?;

        Ok(cluster)
    }

    async fn update(&self, cluster: clusters::Model) -> Result<clusters::Model, Box<dyn Error>> {
        let cluster = cluster
            .into_active_model()
            .update(&self.db.connection)
            .await?;
        Ok(cluster)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        Cluster::delete_by_id(id).exec(&self.db.connection).await?;
        Ok(())
    }
}
