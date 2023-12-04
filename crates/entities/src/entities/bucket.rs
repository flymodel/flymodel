use std::collections::HashMap;

use super::{
    enums::Lifecycle,
    page::{PageInput, Paginated},
};
use crate::{bulk_loader, db::DbLoader, paginated};
use async_graphql::{dataloader::Loader, ComplexObject, SimpleObject};
use chrono::Utc;
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
#[graphql(complex)]
#[graphql(name = "Bucket")]
#[sea_orm(table_name = "bucket")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub namespace: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub region: String,
    pub role: Lifecycle,
    pub shard: i32,
    #[serde(default = "chrono::offset::Utc::now")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(default = "chrono::offset::Utc::now")]
    pub last_modified: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::model_artifacts::Entity")]
    ModelArtifacts,
    #[sea_orm(
        belongs_to = "super::namespace::Entity",
        from = "Column::Namespace",
        to = "super::namespace::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Namespace,
}

impl Related<super::model_artifacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelArtifacts.def()
    }
}

impl Related<super::namespace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::model_artifacts::Entity")]
    ModelArtifacts,
    #[sea_orm(entity = "super::namespace::Entity")]
    Namespace,
}

#[ComplexObject]
impl Model {}

bulk_loader! {
    Model
}

impl DbLoader<Model> {
    pub async fn load_by_namespace(
        &self,
        namespaces: Option<Vec<i32>>,
        roles: Option<Vec<Lifecycle>>,
        page: PageInput,
    ) -> Result<Paginated<Model>, Arc<DbErr>> {
        let mut filters = Entity::find();

        if let Some(namespaces) = namespaces {
            filters = filters.filter(Column::Namespace.is_in(namespaces));
        }

        if let Some(roles) = roles {
            filters = filters.filter(Column::Role.is_in(roles));
        }

        self.load_paginated(filters, page).await
    }
}

paginated! {
    Model,
    Entity
}
