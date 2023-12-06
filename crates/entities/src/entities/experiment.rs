use async_graphql::{ComplexObject, SimpleObject};
use sea_orm::entity::prelude::*;

use crate::{bulk_loader, db::DbLoader, paginated};

use super::page::{PageInput, PaginatedResult};

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
#[graphql(name = "Experiment")]
#[graphql(complex)]
#[sea_orm(table_name = "experiment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub version_id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::experiment_artifact::Entity")]
    ExperimentArtifact,
    #[sea_orm(
        belongs_to = "super::model_version::Entity",
        from = "Column::VersionId",
        to = "super::model_version::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ModelVersion,
}

impl Related<super::experiment_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExperimentArtifact.def()
    }
}

impl Related<super::model_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersion.def()
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
    pub async fn artifacts(
        &self,
        ctx: &async_graphql::Context<'_>,
        page: Option<PageInput>,
    ) -> PaginatedResult<super::experiment_artifact::Model> {
        DbLoader::<super::experiment_artifact::Model>::with_context(ctx)?
            .loader()
            .load_paginated(
                self.find_related(super::experiment_artifact::Entity),
                page.unwrap_or_default(),
            )
            .await
    }
}
