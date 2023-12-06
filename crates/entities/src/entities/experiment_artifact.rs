use async_graphql::{ComplexObject, SimpleObject};
use sea_orm::entity::prelude::*;
use tracing::warn;

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
#[graphql(complex)]
#[graphql(name = "ExperimentArtifact")]
#[sea_orm(table_name = "experiment_artifact")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub experiment_id: i64,
    pub version_id: i64,
    pub blob: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::experiment::Entity",
        from = "Column::ExperimentId",
        to = "super::experiment::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Experiment,
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

impl Related<super::experiment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Experiment.def()
    }
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

#[ComplexObject]
impl Model {
    pub async fn object(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<super::object_blob::Model, async_graphql::Error> {
        self.find_related(super::object_blob::Entity)
            .one(
                &DbLoader::<super::object_blob::Model>::with_context(ctx)?
                    .loader()
                    .db,
            )
            .await.map_err(|err| {
                warn!("error while loading object for experiment artifact: {}", err);
                async_graphql::Error::new("query could not be executed")
            })?
            .ok_or_else(|| {
                warn!("non-deterministic behaviour detected: object not found for experiment artifact {}", self.id);
                async_graphql::Error::new("object not found")
            })
    }
}
