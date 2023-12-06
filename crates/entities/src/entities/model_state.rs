use crate::db::DbLoader;

use super::enums::Lifecycle;
use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
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
    pub id: i64,
    pub version_id: i64,
    pub state: Lifecycle,
    #[serde(default = "chrono::offset::Utc::now")]
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
