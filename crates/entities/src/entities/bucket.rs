use super::enums::Lifecycle;
use async_graphql::{ComplexObject, Interface, SimpleObject};
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
#[graphql(complex)]
#[graphql(name = "Bucket")]
#[sea_orm(table_name = "bucket")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub namespace: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub region: String,
    pub role: Lifecycle,
    pub shard: i32,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub last_modified: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::model_artifacts::Entity")]
    ModelArtifacts,
    #[sea_orm(
        belongs_to = "super::namespace::Entity",
        from = "Column::Namespace",
        to = "super::namespace::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Namespace,
}

impl Related<super::model_artifacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelArtifacts.def()
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
    #[sea_orm(entity = "super::model_artifacts::Entity")]
    ModelArtifacts,
    #[sea_orm(entity = "super::namespace::Entity")]
    Namespace,
}

#[ComplexObject]
impl Model {
    async fn c(&self) -> i32 {
        self.id
    }
}
