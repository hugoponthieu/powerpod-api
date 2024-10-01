use crate::entities;
use async_trait::async_trait;
use entities::{clusters, namespaces};
use sea_orm::prelude::Uuid;
use std::error::Error;

#[async_trait]
pub trait ClusterRepository: Send + Sync {
    async fn m_get(
        &self,
        clutser_ids: Vec<Uuid>,
    ) -> Result<Vec<clusters::Model>, Box<dyn Error + Send + Sync>>;
    async fn get_namespaces(
        &self,
        cluster_id: Uuid,
    ) -> Result<Vec<namespaces::Model>, Box<dyn Error + Send + Sync>>;
    async fn get_all(&self) -> Result<Vec<clusters::Model>, Box<dyn Error + Send + Sync>>;
    async fn get(&self, id: Uuid) -> Result<clusters::Model, Box<dyn Error + Send + Sync>>;
    async fn create(
        &self,
        cluster: clusters::Model,
    ) -> Result<clusters::Model, Box<dyn Error + Send + Sync>>;
    async fn update(
        &self,
        cluster: clusters::Model,
    ) -> Result<clusters::Model, Box<dyn Error + Send + Sync>>;
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>>;
}
