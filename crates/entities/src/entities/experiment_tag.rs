use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

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
#[sea_orm(table_name = "experiment_tag")]
#[graphql(name = "ExperimentTag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub experiment_id: i64,
    pub tag: i64,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::experiment::Entity",
        from = "Column::ExperimentId",
        to = "super::experiment::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Experiment,
    #[sea_orm(
        belongs_to = "super::namespace_tag::Entity",
        from = "Column::Tag",
        to = "super::namespace_tag::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    NamespaceTag,
}

impl Related<super::experiment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Experiment.def()
    }
}

impl Related<super::namespace_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NamespaceTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
