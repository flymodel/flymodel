use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

use crate::{bulk_loader, db::DbLoader, paginated};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    SimpleObject,
    serde::Serialize,
    serde::Deserialize,
)]
#[sea_orm(table_name = "model_version")]
#[graphql(name = "ModelVersion")]

pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub model_id: i64,
    #[sea_orm(column_type = "Text")]
    pub version: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model::Entity",
        from = "Column::ModelId",
        to = "super::model::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Model,
    #[sea_orm(has_many = "super::model_artifact::Entity")]
    ModelArtifact,
    #[sea_orm(has_many = "super::model_state::Entity")]
    ModelState,
}

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Model.def()
    }
}

impl Related<super::model_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelArtifact.def()
    }
}

impl Related<super::model_state::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelState.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

bulk_loader! {
    Model
}

paginated! {
    Model,
    Entity
}

impl DbLoader<Model> {
    pub fn find_by_model_id(&self, sel: Select<Entity>, model_id: i64) -> Select<Entity> {
        sel.filter(Column::ModelId.eq(model_id))
    }

    pub fn find_by_version(&self, sel: Select<Entity>, version: String) -> Select<Entity> {
        sel.filter(Column::Version.like(version))
    }
}
