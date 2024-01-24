use crate::{jsvalue, schema};
use serde::{Deserialize, Serialize};

#[derive(tsify::Tsify, cynic::QueryVariables, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CreateModelVariables {
    pub name: String,
    pub namespace: i32,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateModelVariables")]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct CreateModel {
    #[arguments(namespace: $namespace, name: $name)]
    pub create_model: Model,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub namespace_id: i32,
}

jsvalue! {
    Model,
    CreateModel
}
