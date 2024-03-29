use super::page::{PageInput, PaginatedResult};
use crate::{
    bulk_loader, db::DbLoader, filters::filter_like, paginated, tags_of,
    utils::sql_errs::parse_column_contraint_violation,
};
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use flymodel::{errs::FlymodelError, lifecycle::Lifecycle};
use sea_orm::{entity::prelude::*, ActiveValue};

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
#[sea_orm(table_name = "model")]
#[graphql(name = "Model")]
#[graphql(complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    #[sea_orm(column_name = "namespace")]
    pub namespace_id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::model_tag::Entity")]
    ModelTag,
    #[sea_orm(has_many = "super::model_version::Entity")]
    ModelVersion,
    #[sea_orm(
        belongs_to = "super::namespace::Entity",
        from = "Column::NamespaceId",
        to = "super::namespace::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Namespace,
}

impl Related<super::model_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelTag.def()
    }
}

impl Related<super::model_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersion.def()
    }
}

impl Related<super::namespace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespace.def()
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

#[async_graphql::ComplexObject]
impl Model {
    tags_of! {
        model_tag,
        ModelId
    }

    async fn namespace(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> crate::db::QueryResult<Option<super::namespace::Model>> {
        let loader = DbLoader::<super::namespace::Model>::with_context(ctx)?;
        loader
            .load_one(self.namespace_id.clone())
            .await
            .map_err(|it| FlymodelError::DbLoaderError(it).into_graphql_error())
    }

    async fn versions(
        &self,
        ctx: &async_graphql::Context<'_>,
        version: Option<String>,
        page: Option<PageInput>,
    ) -> PaginatedResult<super::model_version::Model> {
        let db = DbLoader::<super::model_version::Model>::with_context(ctx)?.loader();
        let mut query = super::model_version::Entity::find()
            .filter(super::model_version::Column::ModelId.eq(self.id));
        if let Some(version) = version {
            query = db.find_by_version(query, version);
        }
        db.load_paginated(query, page.unwrap_or_default()).await
    }
}

impl DbLoader<Model> {
    pub fn select_mlmodel_name(&self, sel: Select<Entity>, name: String) -> Select<Entity> {
        filter_like(sel, Column::Name, name)
    }

    pub fn select_mlmodel_namespace(&self, sel: Select<Entity>, ns: Vec<i64>) -> Select<Entity> {
        sel.filter(Column::NamespaceId.is_in(ns))
    }

    pub async fn bulk_paginated_models(&self, page: PageInput) -> PaginatedResult<Model> {
        self.load_paginated(Entity::find(), page).await
    }

    pub async fn find_by_namespace(&self, ns: Vec<i64>, page: PageInput) -> PaginatedResult<Model> {
        self.load_paginated(self.select_mlmodel_namespace(Entity::find(), ns), page)
            .await
    }

    pub async fn find_by_name(&self, name: String, page: PageInput) -> PaginatedResult<Model> {
        self.load_paginated(self.select_mlmodel_name(Entity::find(), name), page)
            .await
    }

    pub async fn find_by_name_and_namespace(
        &self,
        name: Option<String>,
        ns: Option<Vec<i64>>,
        _roles: Option<Vec<Lifecycle>>,
        page: PageInput,
    ) -> PaginatedResult<Model> {
        let mut sel = Entity::find();
        if let Some(name) = name {
            sel = self.select_mlmodel_name(sel, name);
        }
        if let Some(ns) = ns {
            sel = self.select_mlmodel_namespace(sel, ns);
        }

        self.load_paginated(sel, page).await
    }

    pub async fn update_model(&self, id: i64, name: String) -> Result<Model, async_graphql::Error> {
        let model = ActiveModel {
            id: ActiveValue::Set(id),
            name: ActiveValue::Set(name),
            ..Default::default()
        };
        model
            .update(&self.db)
            .await
            .map_err(|err| FlymodelError::DbOperationError(err).into_graphql_error())
    }

    pub async fn delete_model(&self, id: i64) -> Result<bool, async_graphql::Error> {
        let res = Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|err| FlymodelError::DbOperationError(err).into_graphql_error())?;
        Ok(res.rows_affected == 1)
    }

    pub async fn create_model(
        &self,
        namespace: i64,
        name: String,
    ) -> crate::db::QueryResult<Model> {
        let model = ActiveModel {
            name: ActiveValue::Set(name),
            namespace_id: ActiveValue::Set(namespace),
            ..Default::default()
        };
        model.insert(&self.db).await.map_err(|err| {
            match &err.sql_err() {
                Some(SqlErr::ForeignKeyConstraintViolation(source)) => {
                    match parse_column_contraint_violation(source) {
                        Some("model_namespace_fkey") => FlymodelError::ContraintError(format!(
                            "Namespace {namespace} does not exist"
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
