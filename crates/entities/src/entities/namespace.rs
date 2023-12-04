use std::collections::HashMap;

use crate::db::{Database, OrmDataloader};
use async_graphql::{dataloader::Loader, SimpleObject};
use chrono::Utc;
use sea_orm::{
    entity::{prelude::*, *},
    query::*,
};

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
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
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
    Buckets,
    #[sea_orm(has_many = "super::model::Entity")]
    Model,
}

impl Related<super::bucket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Buckets.def()
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
    Buckets,
    #[sea_orm(entity = "super::model::Entity")]
    Model,
}

#[async_trait::async_trait]
impl Loader<i32> for OrmDataloader {
    type Value = Model;
    type Error = std::sync::Arc<DbErr>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Model>, Self::Error> {
        Entity::find()
            .filter(Column::Id.is_in(keys.iter().map(|it| *it as i32).collect::<Vec<_>>()))
            .all(&self.db)
            .await
            .map(|re| HashMap::from_iter(re.iter().map(|it| (it.id as i32, it.to_owned()))))
            .map_err(std::sync::Arc::new)
    }
}
