use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "namespaces")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub cluster_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::clusters::Entity",
        from = "Column::ClusterId",
        to = "super::clusters::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Clusters,
}

impl Related<super::clusters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Clusters.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
