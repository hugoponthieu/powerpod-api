use crate::entities::namespaces::Model as Namespace;
use async_trait::async_trait;
use sea_orm::prelude::Uuid;
use std::{error::Error, sync::Arc};

use crate::{
    repositories::namespace_repository::NamespaceRepository,
    services::namespace_service::NamespaceService,
};

pub struct NamespaceServiceImpl {
    namespace_repository: Arc<Box<dyn NamespaceRepository + Send + Sync>>,
}

impl NamespaceServiceImpl {
    pub fn new(namespace_repository: Arc<Box<dyn NamespaceRepository + Send + Sync>>) -> Self {
        NamespaceServiceImpl {
            namespace_repository,
        }
    }
}

#[async_trait]
impl NamespaceService for NamespaceServiceImpl {
    async fn get(&self, id: Uuid) -> Result<Namespace, Box<dyn Error + Send + Sync>> {
        self.namespace_repository.get(id).await
    }

    async fn mget(&self, ids: Vec<Uuid>) -> Result<Vec<Namespace>, Box<dyn Error + Send + Sync>> {
        self.namespace_repository.mget(ids).await
    }

    async fn get_all(&self) -> Result<Vec<Namespace>, Box<dyn Error + Send + Sync>> {
        self.namespace_repository.get_all().await
    }

    async fn create(
        &self,
        namespace: Namespace,
    ) -> Result<Namespace, Box<dyn Error + Send + Sync>> {
        self.namespace_repository.create(namespace).await
    }

    async fn update(
        &self,
        namespace: Namespace,
    ) -> Result<Namespace, Box<dyn Error + Send + Sync>> {
        self.namespace_repository.update(namespace).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.namespace_repository.delete(id).await
    }
}
