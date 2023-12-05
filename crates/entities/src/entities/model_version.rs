use crate::{bulk_loader, db::DbLoader, paginated};
use async_graphql::{ComplexObject, SimpleObject};
use sea_orm::entity::prelude::*;
use std::sync::Arc;

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
#[graphql(name = "ModelVersion")]
#[graphql(complex)]

pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub model_id: i64,
    #[sea_orm(column_type = "Text")]
    pub version: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::experiment::Entity")]
    Experiment,
    #[sea_orm(has_many = "super::experiment_artifact::Entity")]
    ExperimentArtifact,
    #[sea_orm(
        belongs_to = "super::model::Entity",
        from = "Column::ModelId",
        to = "super::model::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Model,
    #[sea_orm(has_many = "super::model_artifact::Entity")]
    ModelArtifact,
    #[sea_orm(has_many = "super::model_state::Entity")]
    ModelState,
}

impl Related<super::experiment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Experiment.def()
    }
}

impl Related<super::experiment_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExperimentArtifact.def()
    }
}

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Model.def()
    }
}

impl Related<super::model_artifact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelArtifact.def()
    }
}

impl Related<super::model_state::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelState.def()
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
    pub fn find_by_model_id(&self, sel: Select<Entity>, model_id: i64) -> Select<Entity> {
        sel.filter(Column::ModelId.eq(model_id))
    }

    pub fn find_by_version(&self, sel: Select<Entity>, version: String) -> Select<Entity> {
        sel.filter(Column::Version.like(version))
    }
}

#[ComplexObject]
impl Model {
    pub async fn model(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<super::model::Model> {
        super::model::Entity::find_by_id(self.model_id)
            .one(&DbLoader::<Model>::with_context(ctx)?.loader().db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Model not found"))
    }

    pub async fn artifacts(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<super::model_artifact::Model>, Arc<DbErr>> {
        super::model_artifact::Entity::find()
            .filter(super::model_artifact::Column::VersionId.eq(self.id))
            .all(&DbLoader::<Model>::with_context(ctx)?.loader().db)
            .await
            .map_err(Arc::new)
    }

    pub async fn state(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Option<super::model_state::Model>, Arc<DbErr>> {
        super::model_state::Entity::find()
            .filter(super::model_state::Column::VersionId.eq(self.id))
            .one(&DbLoader::<Model>::with_context(ctx)?.loader().db)
            .await
            .map_err(Arc::new)
    }
}
