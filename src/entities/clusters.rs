use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "clusters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::namespaces::Entity")]
    Namespaces,
}

impl Related<super::namespaces::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespaces.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
