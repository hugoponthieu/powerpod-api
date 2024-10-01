use async_trait::async_trait;
use sea_orm::prelude::Uuid;

use crate::entities::clusters::Model as Cluster;
use crate::entities::namespaces::Model as Namespace;
use std::error::Error;

#[async_trait]
pub trait ClusterService {
    async fn get(&self, id: Uuid) -> Result<Cluster, Box<dyn Error + Send + Sync>>;
    async fn mget(&self, ids: Vec<Uuid>) -> Result<Vec<Cluster>, Box<dyn Error + Send + Sync>>;
    async fn get_all(&self) -> Result<Vec<Cluster>, Box<dyn Error + Send + Sync>>;
    async fn get_namespaces(
        &self,
        cluster_id: Uuid,
    ) -> Result<Vec<Namespace>, Box<dyn Error + Send + Sync>>;
    async fn create_cluster(
        &self,
        cluster: Cluster,
    ) -> Result<Cluster, Box<dyn Error + Send + Sync>>;
    async fn update_cluster(
        &self,
        cluster: Cluster,
    ) -> Result<Cluster, Box<dyn Error + Send + Sync>>;
    async fn delete_cluster(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>>;
}
