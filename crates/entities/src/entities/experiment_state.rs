use crate::bulk_loader;

use super::enums::RunState;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
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
#[graphql(name = "ExperimentState")]
#[sea_orm(table_name = "experiment_state")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub experiment_id: i64,
    pub state: RunState,
    pub retry: Option<i32>,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub last_modified: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::experiment::Entity",
        from = "Column::ExperimentId",
        to = "super::experiment::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Experiment,
}

impl Related<super::experiment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Experiment.def()
    }
}

bulk_loader! {
    Model
}

impl ActiveModelBehavior for ActiveModel {}
