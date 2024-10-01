use std::error::Error;
use std::sync::Arc;

use crate::entities::clusters::Model as Cluster;
use crate::entities::namespaces::Model as Namespace;
use crate::{
    repositories::cluster_repository::ClusterRepository, services::cluster_service::ClusterService,
};
use async_trait::async_trait;
use sea_orm::prelude::Uuid;

pub struct ClusterServiceImpl {
    cluster_repository: Arc<Box<dyn ClusterRepository + Send + Sync>>,
}

impl ClusterServiceImpl {
    fn new(cluster_repository: Arc<Box<dyn ClusterRepository + Send + Sync>>) -> Self {
        ClusterServiceImpl { cluster_repository }
    }
}

#[async_trait]

impl ClusterService for ClusterServiceImpl {
    async fn get(&self, id: Uuid) -> Result<Cluster, Box<dyn Error + Send + Sync>> {
        self.cluster_repository.get(id).await
    }

    async fn mget(&self, ids: Vec<Uuid>) -> Result<Vec<Cluster>, Box<dyn Error + Send + Sync>> {
        self.cluster_repository.m_get(ids).await
    }

    async fn get_all(&self) -> Result<Vec<Cluster>, Box<dyn Error + Send + Sync>> {
        self.cluster_repository.get_all().await
    }

    async fn get_namespaces(
        &self,
        cluster_id: Uuid,
    ) -> Result<Vec<Namespace>, Box<dyn Error + Send + Sync>> {
        self.cluster_repository.get_namespaces(cluster_id).await
    }

    async fn create_cluster(
        &self,
        cluster: Cluster,
    ) -> Result<Cluster, Box<dyn Error + Send + Sync>> {
        self.cluster_repository.create(cluster).await
    }

    async fn update_cluster(
        &self,
        cluster: Cluster,
    ) -> Result<Cluster, Box<dyn Error + Send + Sync>> {
        self.cluster_repository.update(cluster).await
    }

    async fn delete_cluster(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.cluster_repository.delete(id).await
    }
}
