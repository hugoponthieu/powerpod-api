use entities::namespaces;

use crate::entities;
pub trait NamespaceRepository {
    fn find_by_id(&self, id: i32) -> Option<namespaces::Model>;
    fn find_by_name(&self, name: &str) -> Option<namespaces::Model>;
    fn find_all(&self) -> Vec<namespaces::Model>;
    fn insert(&self, namespace: namespaces::Model) -> Result<namespaces::Model, String>;
    fn update(&self, namespace: namespaces::Model) -> Result<namespaces::Model, String>;
    fn delete(&self, id: i32) -> Result<(), String>;
}
