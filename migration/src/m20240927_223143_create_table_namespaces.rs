use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Namespaces::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Namespaces::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key()
                            .not_null(),
                    )
                    .col(string(Namespaces::Name).not_null())
                    .col(uuid(Namespaces::ClusterId).not_null())
                    .to_owned(),
            )
            .await?;
        println!("Table created");
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Namespaces::Table, Namespaces::ClusterId)
                    .to(Clusters::Table, Clusters::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Namespaces::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Namespaces {
    Table,
    Id,
    Name,
    ClusterId,
}

#[derive(DeriveIden)]
enum Clusters {
    Table,
    Id,
}
