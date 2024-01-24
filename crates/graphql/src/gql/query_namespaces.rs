use crate::schema;

use crate::{fragments::*, jsvalue};
use serde::{Deserialize, Serialize};

#[derive(tsify::Tsify, cynic::QueryVariables, Debug, Clone, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub struct QueryNamespacesVariables {
    pub page: Option<Page>,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "QueryNamespacesVariables")]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct QueryNamespaces {
    #[arguments(page: $page)]
    pub namespace: PaginatedNamespace,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub struct PaginatedNamespace {
    pub page: CurrentPage,
    pub total_pages: i32,
    pub total_items: i32,
    pub data: Vec<Namespace>,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub struct Namespace {
    pub id: i32,
    pub name: String,
    pub description: String,
}

jsvalue! {
    Namespace,
    PaginatedNamespace,
    QueryNamespaces
}
