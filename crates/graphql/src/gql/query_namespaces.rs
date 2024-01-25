use crate::schema;

use crate::{fragments::*, jsvalue};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(tsify::Tsify, cynic::QueryVariables, Debug, Clone, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub struct QueryNamespacesVariables {
    pub page: Option<Page>,
}

crate::new_for! {
    QueryNamespacesVariables,
    page: Option<Page>,
}

#[hybrid_feature_class(python = true)]
#[derive(tsify::Tsify, cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "QueryNamespacesVariables")]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct QueryNamespaces {
    #[arguments(page: $page)]
    pub namespace: PaginatedNamespace,
}

#[hybrid_feature_class(python = true)]
#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]

pub struct PaginatedNamespace {
    pub page: CurrentPage,
    pub total_pages: i32,
    pub total_items: i32,
    pub data: Vec<Namespace>,
}

#[hybrid_feature_class(python = true)]
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
