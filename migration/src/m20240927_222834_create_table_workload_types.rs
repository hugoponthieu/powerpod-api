use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkloadTypes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkloadTypes::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key()
                            .not_null(),
                    )
                    .col(string(WorkloadTypes::Type).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkloadTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WorkloadTypes {
    Table,
    Id,
    Type,
}
