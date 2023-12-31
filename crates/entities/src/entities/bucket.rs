use super::page::{PageInput, PaginatedResult};
use crate::{
    bulk_loader,
    db::{DbLoader, QueryResult},
    paginated,
    utils::sql_errs::parse_column_contraint_violation,
};
use async_graphql::{ComplexObject, SimpleObject};
use chrono::Utc;
use flymodel::{errs::FlymodelError, lifecycle::Lifecycle};
use sea_orm::{entity::prelude::*, ActiveValue, Select};
use tracing::debug;

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
#[sea_orm(table_name = "bucket")]
#[graphql(complex)]
#[graphql(name = "Bucket")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub namespace: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub region: String,
    pub role: Lifecycle,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub last_modified: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::namespace::Entity",
        from = "Column::Namespace",
        to = "super::namespace::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Namespace,
    #[sea_orm(has_many = "super::object_blob::Entity")]
    ObjectBlob,
}

impl Related<super::namespace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespace.def()
    }
}

impl Related<super::object_blob::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ObjectBlob.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[ComplexObject]
impl Model {}

bulk_loader! {
    Model
}

paginated! {
    Model,
    Entity
}

impl DbLoader<Model> {
    pub async fn find_by_namespace(
        &self,
        namespaces: Option<Vec<i64>>,
        roles: Option<Vec<Lifecycle>>,
        page: PageInput,
    ) -> PaginatedResult<Model> {
        let mut filters = Entity::find();

        if let Some(namespaces) = namespaces {
            filters = filters.filter(Column::Namespace.is_in(namespaces));
        }

        if let Some(roles) = roles {
            filters = filters.filter(Column::Role.is_in(roles));
        }

        self.load_paginated(filters, page).await
    }

    pub async fn create_bucket(
        &self,
        namespace: i64,
        name: String,
        region: Option<String>,
        role: Lifecycle,
    ) -> QueryResult<Model> {
        let bucket = ActiveModel {
            namespace: ActiveValue::Set(namespace),
            name: ActiveValue::Set(name.clone()),
            region: ActiveValue::Set(region.unwrap_or_default()),
            role: ActiveValue::Set(role),
            ..Default::default()
        };
        debug!("creating bucket: {:#?}", bucket);
        bucket.insert(&self.db).await.map_err(|err| {
            tracing::error!("insert err: {:#?}", &err.sql_err());
            match &err.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(source)) => {
                    let source = parse_column_contraint_violation(source);
                    match source {
                        Some("bucket_name_idx") => FlymodelError::ContraintError(format!(
                            "Bucket names may not be reused unless via a different region: {name}",
                        )),
                        _ => FlymodelError::DbOperationError(err),
                    }
                }
                _ => FlymodelError::DbOperationError(err),
            }
            .into_graphql_error()
        })
    }
}
