use crate::db::DbLoader;

use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};
use flymodel::{errs::FlymodelError, lifecycle::Lifecycle};
use sea_orm::{entity::prelude::*, ActiveValue, IntoActiveModel};
use tracing::warn;

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
#[sea_orm(table_name = "model_state")]
#[graphql(complex)]
#[graphql(name = "ModelState")]

pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub version_id: i64,
    pub state: Lifecycle,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model_version::Entity",
        from = "Column::VersionId",
        to = "super::model_version::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ModelVersion,
}

impl Related<super::model_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::model_version::Entity")]
    ModelVersion,
}

#[ComplexObject]
impl Model {
    async fn version(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<super::model_version::Model, async_graphql::Error> {
        super::model_version::Entity::find_by_id(self.version_id)
            .one(&DbLoader::<Model>::with_context(ctx)?.loader().db)
            .await?
            .ok_or_else(|| {
                warn!("non deterministic behaviour detected");
                async_graphql::Error::new("Model not found")
            })
    }
}

impl DbLoader<Model> {
    pub async fn update_state(
        &self,
        version_id: i64,
        state: Lifecycle,
    ) -> Result<Model, async_graphql::Error> {
        let current = Entity::find()
            .filter(Column::VersionId.eq(version_id))
            .one(&self.db)
            .await
            .map_err(|err| {
                FlymodelError::DbLoaderError(std::sync::Arc::new(err)).into_graphql_error()
            })?;
        let model = current
            .ok_or_else(|| FlymodelError::InvalidResourceId(version_id).into_graphql_error())?;
        let current_state = model.state;
        if current_state == state {
            return Ok(model);
        }

        let mut active = model.into_active_model();
        if current_state < state {
            match (current_state, state) {
                (Lifecycle::Test, Lifecycle::Qa)
                | (Lifecycle::Qa, Lifecycle::Stage)
                | (Lifecycle::Stage, _) => {
                    active.state = ActiveValue::Set(state);
                }
                _ => {
                    return Err(FlymodelError::InvalidTransition {
                        current: current_state,
                        requested: state,
                    }
                    .into_graphql_error())
                }
            }
        } else if current_state == Lifecycle::Prod {
            return Err(FlymodelError::InvalidTransition {
                current: current_state,
                requested: state,
            }
            .into_graphql_error());
        } else {
            active.state = ActiveValue::Set(state)
        }

        let model = active.update(&self.db).await?;
        Ok(model)
    }
}
