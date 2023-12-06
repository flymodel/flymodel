use crate::{bulk_loader, paginated};

use super::enums::{ArchiveEncoding, ArchiveFormat};
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
#[sea_orm(table_name = "object_blob")]
#[graphql(name = "ObjectBlob")]
// #[graphql(complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub bucket_id: i64,
    #[sea_orm(column_type = "Text")]
    pub key: String,
    pub version_id: String,
    pub size: i64,
    pub sha256: String,
    pub archive: Option<ArchiveFormat>,
    pub encode: Option<ArchiveEncoding>,
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
    Bucket,
    #[sea_orm(has_many = "super::experiment_artifact::Entity")]
    ExperimentArtifact,
    #[sea_orm(has_many = "super::model_artifact::Entity")]
    ModelArtifact,
}

impl Related<super::bucket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bucket.def()
    }
}

impl Related<super::experiment_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExperimentArtifact.def()
    }
}

impl Related<super::model_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelArtifact.def()
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
