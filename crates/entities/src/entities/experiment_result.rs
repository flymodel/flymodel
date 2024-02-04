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
#[graphql(name = "ExperimentResult")]
#[sea_orm(table_name = "experiment_result")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub experiment_id: i64,
    pub state: RunState,
    pub retries: i32,
    pub duration_ms: i64,
    #[serde(skip_deserializing, default = "chrono::offset::Utc::now")]
    pub finished_at: DateTime<Utc>,
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

impl ActiveModelBehavior for ActiveModel {}
