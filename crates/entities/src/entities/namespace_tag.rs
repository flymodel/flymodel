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
#[sea_orm(table_name = "namespace_tag")]
#[graphql(name = "NamespaceTag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub namespace_id: i64,
    #[sea_orm(column_type = "Text")]
    pub tag: String,
    #[sea_orm(column_type = "Text")]
    pub color: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::experiment_tag::Entity")]
    ExperimentTag,
    #[sea_orm(has_many = "super::model_tag::Entity")]
    ModelTag,
    #[sea_orm(has_many = "super::model_version_tag::Entity")]
    ModelVersionTag,
    #[sea_orm(
        belongs_to = "super::namespace::Entity",
        from = "Column::NamespaceId",
        to = "super::namespace::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Namespace,
}

impl Related<super::experiment_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExperimentTag.def()
    }
}

impl Related<super::model_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelTag.def()
    }
}

impl Related<super::model_version_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersionTag.def()
    }
}

impl Related<super::namespace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
