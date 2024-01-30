use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};
use flymodel::errs::FlymodelError;
use sea_orm::{entity::prelude::*, ActiveValue};

use crate::{bulk_loader, db::DbLoader, filters::filter_like, paginated};

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
    #[serde(skip_deserializing)]
    pub id: i64,
    pub version_id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::experiment_artifact::Entity")]
    ExperimentArtifact,
    #[sea_orm(has_many = "super::experiment_tag::Entity")]
    ExperimentTag,
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

impl DbLoader<Model> {
    pub async fn bulk_paginated_experiments(
        &self,
        name: Option<String>,
        version_id: Option<i64>,
        page: PageInput,
    ) -> PaginatedResult<Model> {
        let mut query = Entity::find();
        if let Some(name) = name {
            query = Self::find_by_name(query, name);
        }
        if let Some(version_id) = version_id {
            query = Self::model_version(query, version_id);
        }

        self.load_paginated(query, page).await
    }

    pub fn find_by_name(sel: sea_orm::Select<Entity>, name: String) -> sea_orm::Select<Entity> {
        filter_like(sel, Column::Name, name)
    }

    pub fn model_version(sel: sea_orm::Select<Entity>, version_id: i64) -> sea_orm::Select<Entity> {
        sel.filter(Column::VersionId.eq(version_id))
    }

    pub async fn delete_experiment(&self, id: i64) -> Result<bool, async_graphql::Error> {
        let res = Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|err| FlymodelError::DbOperationError(err).into_graphql_error())?;
        Ok(res.rows_affected == 1)
    }

    pub async fn create_experiment(
        &self,
        version_id: i64,
        name: String,
    ) -> Result<Model, async_graphql::Error> {
        let active_model = ActiveModel {
            version_id: ActiveValue::Set(version_id),
            name: ActiveValue::Set(name),
            ..Default::default()
        };
        active_model
            .insert(&self.db)
            .await
            .map_err(|err| FlymodelError::DbOperationError(err).into_graphql_error())
    }

    pub async fn single_model_version(
        &self,
        experiment_id: i64,
    ) -> Result<Option<(super::model_version::Model, Model)>, FlymodelError> {
        let ents = Entity::find()
            .filter(Column::Id.eq(experiment_id))
            .find_also_related(super::model_version::Entity)
            .one(&self.db)
            .await
            .map_err(|err| FlymodelError::DbLoaderError(std::sync::Arc::new(err)))?;

        if let Some((experiment, version)) = ents {
            if let Some(version) = version {
                Ok(Some((version, experiment)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
