use crate::{jsvalue, scalars::*, schema};
use flymodel_macros::hybrid_feature_class;

use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(from_wasm_abi))]
pub struct CreateNamespaceVariables {
    pub name: String,
    pub description: String,
}

crate::new_for! {
    CreateNamespaceVariables,
    name: &str,
    description: &str,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
#[cynic(graphql_type = "Mutation", variables = "CreateNamespaceVariables")]
pub struct CreateNamespace {
    #[arguments(name: $name, description: $description)]
    pub create_namespace: Namespace,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct Namespace {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: DateTime,
    pub last_modified: DateTime,
}

jsvalue! {
    Namespace,
    CreateNamespace
}
