use crate::{bulk_loader, db::DbLoader, paginated};
use async_graphql::{Context, SimpleObject};
use chrono::Utc;
use sea_orm::entity::prelude::*;
use std::sync::Arc;

use super::page::{PageInput, Paginated, PaginatedResult};

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
#[sea_orm(table_name = "namespace")]
#[graphql(name = "Namespace")]
#[graphql(complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[serde(default = "chrono::offset::Utc::now")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(default = "chrono::offset::Utc::now")]
    pub last_modified: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bucket::Entity")]
    Bucket,
    #[sea_orm(has_many = "super::model::Entity")]
    Model,
}

impl Related<super::bucket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bucket.def()
    }
}

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Model.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::bucket::Entity")]
    Bucket,
    #[sea_orm(entity = "super::model::Entity")]
    Model,
}

bulk_loader! {
    Model
}

paginated! {
    Model,
    Entity
}

impl DbLoader<Model> {
    pub async fn bulk_paginated_namespaces(
        &self,
        page: PageInput,
    ) -> Result<Paginated<Model>, Arc<DbErr>> {
        self.load_paginated(Entity::find(), page).await
    }
}

#[async_graphql::ComplexObject]
impl Model {
    async fn buckets<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        page: Option<PageInput>,
    ) -> PaginatedResult<super::bucket::Model> {
        DbLoader::<super::bucket::Model>::with_context(ctx)?
            .loader()
            .find_by_namespace(Some(vec![self.id]), None, page.unwrap_or_default())
            .await
    }

    async fn models<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        page: Option<PageInput>,
    ) -> PaginatedResult<super::model::Model> {
        DbLoader::<super::model::Model>::with_context(ctx)?
            .loader()
            .find_by_namespace(vec![self.id], page.unwrap_or_default())
            .await
    }
}
