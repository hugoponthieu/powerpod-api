pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_clusters;
mod m20240927_222834_create_table_workload_types;
mod m20240927_223143_create_table_namespaces;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_clusters::Migration),
            Box::new(m20240927_222834_create_table_workload_types::Migration),
            Box::new(m20240927_223143_create_table_namespaces::Migration),
        ]
    }
}
