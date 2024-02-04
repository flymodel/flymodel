use async_graphql::{ComplexObject, SimpleObject};
use sea_orm::entity::prelude::*;

use crate::{bulk_loader, paginated, tags_meta};

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
#[sea_orm(table_name = "model_version_tag")]
#[graphql(name = "ModelVersionTag", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub version_id: i64,
    pub tag: i64,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model_version::Entity",
        from = "Column::VersionId",
        to = "super::model_version::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ModelVersion,
    #[sea_orm(
        belongs_to = "super::namespace_tag::Entity",
        from = "Column::Tag",
        to = "super::namespace_tag::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    NamespaceTag,
}

impl Related<super::model_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersion.def()
    }
}

impl Related<super::namespace_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NamespaceTag.def()
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
    tags_meta! {}
}
