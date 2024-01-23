use crate::{
    bulk_loader, db::DbLoader, paginated, utils::sql_errs::parse_column_contraint_violation,
};
use async_graphql::{ComplexObject, SimpleObject};
use flymodel::{errs::FlymodelError, lifecycle::Lifecycle};
use sea_orm::{entity::prelude::*, ActiveValue};

use tracing::warn;

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
#[sea_orm(table_name = "model_version")]
#[graphql(name = "ModelVersion")]
#[graphql(complex)]

pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
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
    #[sea_orm(has_many = "super::model_version_tag::Entity")]
    ModelVersionTag,
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

impl Related<super::model_version_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersionTag.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_save<'a, C>(model: Model, db: &'a C, insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait + 'a,
    {
        if insert {
            let sync = super::model_state::ActiveModel {
                version_id: ActiveValue::Set(model.id),
                state: ActiveValue::Set(Lifecycle::Test),
                ..Default::default()
            };
            sync.insert(db).await?;
        }
        Ok(model)
    }
}

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

    pub async fn create_version(
        &self,
        model: i64,
        version: String,
    ) -> Result<Model, async_graphql::Error> {
        let version = ActiveModel {
            model_id: ActiveValue::Set(model),
            version: ActiveValue::Set(version),
            ..Default::default()
        };
        version.insert(&self.db).await.map_err(|err| {
            match &err.sql_err() {
                Some(SqlErr::ForeignKeyConstraintViolation(source)) => {
                    match parse_column_contraint_violation(source) {
                        Some("model_version_model_id_fkey") => FlymodelError::ContraintError(
                            format!("The given model does not exist: {model}"),
                        ),
                        _ => FlymodelError::DbOperationError(err),
                    }
                }
                _ => FlymodelError::DbOperationError(err),
            }
            .into_graphql_error()
        })
    }

    pub async fn state(
        &self,
        ent: &Model,
    ) -> Result<Option<super::model_state::Model>, FlymodelError> {
        ent.find_related(super::model_state::Entity)
            .one(&self.db)
            .await
            .map_err(|err| FlymodelError::DbOperationError(err))
    }

    // pub async fn namespace(&self, ent: &Model,) {
    //     ent.;
    // }
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
            .ok_or_else(|| {
                warn!("non deterministic behaviour detected");
                async_graphql::Error::new("Model not found")
            })
    }

    pub async fn artifacts(
        &self,
        ctx: &async_graphql::Context<'_>,
        page: Option<PageInput>,
    ) -> PaginatedResult<super::model_artifact::Model> {
        DbLoader::<super::model_artifact::Model>::with_context(ctx)?
            .loader()
            .load_paginated(
                self.find_related(super::model_artifact::Entity),
                page.unwrap_or_default(),
            )
            .await
    }

    pub async fn experiments(
        &self,
        ctx: &async_graphql::Context<'_>,
        page: Option<PageInput>,
    ) -> PaginatedResult<super::experiment::Model> {
        DbLoader::<super::experiment::Model>::with_context(ctx)?
            .loader()
            .load_paginated(
                self.find_related(super::experiment::Entity),
                page.unwrap_or_default(),
            )
            .await
    }

    pub async fn state(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> crate::db::QueryResult<Option<super::model_state::Model>> {
        DbLoader::<Model>::with_context(ctx)?
            .loader()
            .state(self)
            .await
            .map_err(|err| err.into_graphql_error())
    }

    async fn tags(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> crate::db::QueryResult<Vec<super::model_version_tag::Model>> {
        self.find_related(super::model_version_tag::Entity)
            .all(
                &DbLoader::<super::model_version_tag::Model>::with_context(ctx)?
                    .loader()
                    .db,
            )
            .await
            .map_err(|it| FlymodelError::DbOperationError(it).into_graphql_error())
    }
}
