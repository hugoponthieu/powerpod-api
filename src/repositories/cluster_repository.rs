use std::error::Error;

use crate::entities;
use entities::{clusters, namespaces};
use sea_orm::prelude::Uuid;

pub trait ClusterRepository {
    async fn m_get(&self, clutser_ids: Vec<Uuid>) -> Result<Vec<clusters::Model>, Box<dyn Error>>;
    async fn get_namespaces(
        &self,
        cluster_id: Uuid,
    ) -> Result<Vec<namespaces::Model>, Box<dyn Error>>;
    async fn get_all(&self) -> Result<Vec<clusters::Model>, Box<dyn Error>>;
    async fn get(&self, id: Uuid) -> Result<clusters::Model, Box<dyn Error>>;
    async fn create(&self, cluster: clusters::Model) -> Result<clusters::Model, Box<dyn Error>>;
    async fn update(&self, cluster: clusters::Model) -> Result<clusters::Model, Box<dyn Error>>;
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error>>;
}
