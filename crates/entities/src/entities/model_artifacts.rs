use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    SimpleObject,
)]
#[sea_orm(table_name = "model_artifacts")]
#[graphql(name = "ModelArtifacts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub version_id: i32,
    pub bucket_id: i32,
    #[sea_orm(column_type = "Text")]
    pub key: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bucket::Entity",
        from = "Column::BucketId",
        to = "super::bucket::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Buckets,
    #[sea_orm(
        belongs_to = "super::model_version::Entity",
        from = "Column::VersionId",
        to = "super::model_version::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ModelVersion,
}

impl Related<super::bucket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Buckets.def()
    }
}

impl Related<super::model_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::bucket::Entity")]
    Buckets,
    #[sea_orm(entity = "super::model_version::Entity")]
    ModelVersion,
}
