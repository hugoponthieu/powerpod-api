use crate::entities::namespaces::Model as Namespace;
use async_trait::async_trait;
use sea_orm::prelude::Uuid;
use std::error::Error;

#[async_trait]
pub trait NamespaceService {
    async fn get(&self, id: Uuid) -> Result<Namespace, Box<dyn Error + Send + Sync>>;
    async fn mget(&self, ids: Vec<Uuid>) -> Result<Vec<Namespace>, Box<dyn Error + Send + Sync>>;
    async fn get_all(&self) -> Result<Vec<Namespace>, Box<dyn Error + Send + Sync>>;
    async fn create(
        &self,
        namespace: Namespace,
    ) -> Result<Namespace, Box<dyn Error + Send + Sync>>;
    async fn update(
        &self,
        namespace: Namespace,
    ) -> Result<Namespace, Box<dyn Error + Send + Sync>>;
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>>;
}
