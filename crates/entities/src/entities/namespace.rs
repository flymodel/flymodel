use crate::{bulk_loader, db::DbLoader, filters::filter_like, paginated};
use async_graphql::{dataloader::DataLoader, Context, SimpleObject};
use chrono::Utc;

use flymodel::errs::FlymodelError;
use sea_orm::{entity::prelude::*, ActiveValue};
use tracing::debug;

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
#[sea_orm(table_name = "namespace")]
#[graphql(name = "Namespace")]
#[graphql(complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
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
        name: Option<String>,
        page: PageInput,
    ) -> PaginatedResult<Model> {
        let mut query = Entity::find();
        if let Some(name) = name {
            query = Self::find_by_name(query, name);
        }
        self.load_paginated(query, page).await
    }

    pub async fn namespace_of_model(
        this: &DataLoader<Self>,
        model: &super::model_version::Model,
    ) -> Result<Option<Model>, FlymodelError> {
        let model = model
            .find_related(super::model::Entity)
            .one(&this.loader().db)
            .await
            .map_err(|err| FlymodelError::DbLoaderError(std::sync::Arc::new(err)))?;
        if let Some(model) = model {
            this.load_one(model.namespace_id.clone())
                .await
                .map_err(|it| FlymodelError::DbLoaderError(it))
        } else {
            Err(FlymodelError::NonDeterministicError(
                "Expected model for model version".into(),
            ))
        }
    }

    #[inline]
    pub fn find_by_name(sel: Select<Entity>, name: String) -> Select<Entity> {
        filter_like(sel, Column::Name, name)
    }

    pub async fn update_namespace(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Model, async_graphql::Error> {
        let mut ns = ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };
        if let Some(name) = name {
            ns.name = ActiveValue::Set(name);
        }
        if let Some(description) = description {
            ns.description = ActiveValue::Set(description);
        }
        ns.update(&self.db)
            .await
            .map_err(|it| FlymodelError::DbOperationError(it).into_graphql_error())
    }

    pub async fn delete_namespace(&self, id: i64) -> Result<bool, async_graphql::Error> {
        let res = Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|err| FlymodelError::DbOperationError(err).into_graphql_error())?;
        Ok(res.rows_affected == 1)
    }

    pub async fn create_namespace<'ctx>(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<Model, async_graphql::Error> {
        let mut ns = ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        };
        if let Some(description) = description {
            ns.description = ActiveValue::Set(description);
        }
        debug!("creating namespace: {:#?}", ns);
        ns.insert(&self.db)
            .await
            .map_err(|it| FlymodelError::DbOperationError(it).into_graphql_error())
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
