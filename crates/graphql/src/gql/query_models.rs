use crate::{fragments::*, jsvalue, scalars::*, schema};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(tsify::Tsify, cynic::QueryVariables, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct NamespaceModelsVariables {
    pub model_id: Option<i32>,
    pub model_name: Option<String>,
    pub model_namespace: Option<i32>,
    pub page: Option<Page>,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "NamespaceModelsVariables")]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct NamespaceModels {
    #[arguments(id: $model_id, page: $page, name: $model_name, namespace: $model_namespace)]
    pub model: PaginatedModel,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi)]
pub struct PaginatedModel {
    pub page: CurrentPage,
    pub total_pages: i32,
    pub total_items: i32,
    pub data: Vec<Model>,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Model {
    pub id: i32,
    pub name: String,
    #[tsify(type = "string")]
    pub created_at: DateTime,
    #[tsify(type = "string")]
    pub last_modified: DateTime,
    pub namespace_id: i32,
}

jsvalue! {
    Model,
    PaginatedModel,
    NamespaceModels
}
