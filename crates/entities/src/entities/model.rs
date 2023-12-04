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
#[sea_orm(table_name = "model")]
#[graphql(name = "Model")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub namespace: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub created_at: DateTimeWithTimeZone,
    pub last_modified: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::model_version::Entity")]
    ModelVersion,
    #[sea_orm(
        belongs_to = "super::namespace::Entity",
        from = "Column::Namespace",
        to = "super::namespace::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Namespace,
}

impl Related<super::model_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersion.def()
    }
}

impl Related<super::namespace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::model_version::Entity")]
    ModelVersion,
    #[sea_orm(entity = "super::namespace::Entity")]
    Namespace,
}
