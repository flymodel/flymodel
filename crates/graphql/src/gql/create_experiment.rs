use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(from_wasm_abi))]
pub struct CreateExperimentVariables {
    pub experiment_name: String,
    pub model_version_id: i32,
}

crate::new_for! {
    CreateExperimentVariables,
    experiment_name: &str,
    model_version_id: i32,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateExperimentVariables")]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct CreateExperiment {
    #[arguments(modelVersion: $model_version_id, name: $experiment_name)]
    pub create_experiment: Experiment,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct Experiment {
    pub id: i32,
    pub name: String,
    pub version_id: i32,
}

jsvalue! {
    Experiment,
    CreateExperiment
}
