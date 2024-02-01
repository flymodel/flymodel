use async_graphql::{ComplexObject, SimpleObject};
use flymodel::errs::FlymodelError;
use sea_orm::{entity::prelude::*, ActiveValue, DatabaseTransaction};
use tracing::warn;

use crate::{bulk_loader, db::DbLoader, paginated, utils::handle::constraint_or_db_operational};

use super::upload::UploadBlobRequestParams;

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
#[graphql(name = "ModelArtifact")]
#[graphql(complex)]
#[sea_orm(table_name = "")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub version_id: i64,
    pub blob: i64,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub extra: Option<Json>,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model_version::Entity",
        from = "Column::VersionId",
        to = "super::model_version::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ModelVersion,
    #[sea_orm(
        belongs_to = "super::object_blob::Entity",
        from = "Column::Blob",
        to = "super::object_blob::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ObjectBlob,
}

impl Related<super::model_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersion.def()
    }
}

impl Related<super::object_blob::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ObjectBlob.def()
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
    pub async fn create_new_artifact(
        conn: &DatabaseTransaction,
        version: &super::model_version::Model,
        blob: &super::object_blob::Model,
        args: &UploadBlobRequestParams,
        extra: Option<Json>,
    ) -> Result<Model, FlymodelError> {
        let this = ActiveModel {
            version_id: ActiveValue::Set(version.id),
            name: ActiveValue::Set(args.artifact_name.clone()),
            blob: ActiveValue::Set(blob.id),
            extra: ActiveValue::Set(extra),
            id: ActiveValue::NotSet,
        };
        Ok(Entity::insert(this)
            .exec_with_returning(conn)
            .await
            .map_err(|err| {
                constraint_or_db_operational(
                    "model_artifact_version_idx",
                    err,
                    format!(
                        "Artifact with name {name} for the model {id} already exists.",
                        name = args.artifact_name,
                        id = version.id
                    ),
                )
            })?)
    }
}

#[ComplexObject]
impl Model {
    pub async fn object(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<super::object_blob::Model> {
        DbLoader::<super::object_blob::Model>::with_context(ctx)?
            .load_one(self.blob)
            .await?
            .ok_or_else(|| {
                warn!(
                    "non deterministic behaviour detected. expected blob reference but found null."
                );
                async_graphql::Error::new("Blob not found")
            })
    }
}
