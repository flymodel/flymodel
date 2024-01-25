use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(tsify::Tsify, cynic::QueryVariables, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CreateModelVariables {
    pub name: String,
    pub namespace: i32,
}

crate::new_for! {
    CreateModelVariables,
    name: &str,
    namespace: i32,
}

#[hybrid_feature_class(python = true)]
#[derive(tsify::Tsify, cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateModelVariables")]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct CreateModel {
    #[arguments(namespace: $namespace, name: $name)]
    pub create_model: Model,
}

#[hybrid_feature_class(python = true)]
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
