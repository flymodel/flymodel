use crate::{jsvalue, schema};
use serde::{Deserialize, Serialize};

#[derive(tsify::Tsify, cynic::QueryVariables, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CreateExperimentVariables {
    pub experiment_name: String,
    pub model_version_id: i32,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateExperimentVariables")]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct CreateExperiment {
    #[arguments(modelVersion: $model_version_id, name: $experiment_name)]
    pub create_experiment: Experiment,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub struct Experiment {
    pub id: i32,
    pub name: String,
    pub version_id: i32,
}

jsvalue! {
    Experiment,
    CreateExperiment
}
