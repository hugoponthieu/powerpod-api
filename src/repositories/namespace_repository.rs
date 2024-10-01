use async_trait::async_trait;
use std::error::Error;

use crate::entities::namespaces::Model as Namespace;
use sea_orm::prelude::Uuid;

#[async_trait]
pub trait NamespaceRepository {
    async fn get(&self, id: Uuid) -> Result<Namespace, Box<dyn Error + Send + Sync>>;
    async fn mget(&self, ids: Vec<Uuid>) -> Result<Vec<Namespace>, Box<dyn Error + Send + Sync>>;
    async fn get_all(&self) -> Result<Vec<Namespace>, Box<dyn Error + Send + Sync>>;
    async fn create(&self, namespace: Namespace)
        -> Result<Namespace, Box<dyn Error + Send + Sync>>;
    async fn update(&self, namespace: Namespace)
        -> Result<Namespace, Box<dyn Error + Send + Sync>>;
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>>;
}
