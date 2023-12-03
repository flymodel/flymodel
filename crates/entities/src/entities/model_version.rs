use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

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
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub model_id: i32,
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
    #[sea_orm(has_many = "super::model_artifacts::Entity")]
    ModelArtifacts,
    #[sea_orm(has_many = "super::model_states::Entity")]
    ModelStates,
}

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Model.def()
    }
}

impl Related<super::model_artifacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelArtifacts.def()
    }
}

impl Related<super::model_states::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelStates.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::model::Entity")]
    Model,
    #[sea_orm(entity = "super::model_artifacts::Entity")]
    ModelArtifacts,
    #[sea_orm(entity = "super::model_states::Entity")]
    ModelStates,
}
