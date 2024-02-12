use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use partial_context::PartialContext;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize, PartialContext)]
#[context_needs(
    #[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)],
    #[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
)]
pub struct CreateExperimentVariables {
    pub experiment_name: String,
    #[context]
    pub model_version_id: i32,
}

crate::new_for! {
    CreateExperimentVariables,
    experiment_name: &str,
    model_version_id: i32,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateExperimentVariables")]
pub struct CreateExperiment {
    #[arguments(modelVersion: $model_version_id, name: $experiment_name)]
    pub create_experiment: Experiment,
}

#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
pub struct Experiment {
    pub id: i32,
    pub name: String,
    pub version_id: i32,
}

jsvalue! {
    Experiment,
    CreateExperiment
}
