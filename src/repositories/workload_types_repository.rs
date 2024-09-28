use crate::entities::workload_types;

pub trait WorkloadTypesRepository {
    fn find_by_id(&self, id: i32) -> Option<workload_types::Model>;
    fn find_by_name(&self, name: &str) -> Option<workload_types::Model>;
    fn find_all(&self) -> Vec<workload_types::Model>;
    fn insert(&self, workload_type: workload_types::Model)
        -> Result<workload_types::Model, String>;
    fn update(&self, workload_type: workload_types::Model)
        -> Result<workload_types::Model, String>;
    fn delete(&self, id: i32) -> Result<(), String>;
}
