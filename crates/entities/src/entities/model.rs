use super::{
    enums::Lifecycle,
    page::{PageInput, PaginatedResult},
};
use crate::{bulk_loader, db::DbLoader, paginated};
use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, QuerySelect};
use sea_query::{Expr, Func};

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
    pub id: i64,
    #[sea_orm(column_name = "namespace")]
    pub namespace_id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub created_at: DateTimeWithTimeZone,
    pub last_modified: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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
    async fn namespace(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> crate::db::QueryResult<Option<super::namespace::Model>> {
        let loader = DbLoader::<super::namespace::Model>::context(ctx)?;
        loader.load_one(self.namespace_id.clone()).await
    }
}

impl DbLoader<Model> {
    pub fn select_mlmodel_name(&self, sel: Select<Entity>, name: String) -> Select<Entity> {
        sel.expr(Func::lower(Expr::col(Column::Name)))
            .filter(Column::Name.like(name.to_lowercase()))
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
}
