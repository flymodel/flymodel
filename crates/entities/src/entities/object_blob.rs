use crate::{bulk_loader, db::DbLoader, paginated};

use super::{
    enums::{ArchiveEncoding, ArchiveFormat},
    upload::UploadBlobRequestParams,
};

use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use flymodel::errs::FlymodelError;
use sea_orm::{entity::prelude::*, ActiveValue};

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
    #[serde(skip_deserializing)]
    pub id: i64,
    pub bucket_id: i64,
    #[sea_orm(column_type = "Text")]
    pub key: String,
    pub version_id: String,
    pub size: i64,
    pub sha256: String,
    pub archive: Option<ArchiveFormat>,
    pub encode: Option<ArchiveEncoding>,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub created_at: DateTime<Utc>,
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

impl DbLoader<Model> {
    pub async fn create_new_blob(
        &self,
        bucket_id: i64,
        key: String,
        version_id: String,
        args: &UploadBlobRequestParams,
        size: i64,
        sha256: String,
    ) -> Result<Model, FlymodelError> {
        let this = ActiveModel {
            bucket_id: ActiveValue::Set(bucket_id),
            key: ActiveValue::Set(key),
            version_id: ActiveValue::Set(version_id),
            archive: ActiveValue::Set(args.archive),
            encode: ActiveValue::Set(args.encode),
            created_at: ActiveValue::Set(Utc::now()),
            size: ActiveValue::Set(size),
            sha256: ActiveValue::Set(sha256),
            ..Default::default()
        };

        this.insert(&self.db)
            .await
            .map_err(|err| FlymodelError::DbOperationError(err))
    }
}
