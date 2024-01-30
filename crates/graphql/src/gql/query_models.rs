use crate::{fragments::*, jsvalue, scalars::*, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(from_wasm_abi))]
pub struct NamespaceModelsVariables {
    pub model_id: Option<i32>,
    pub model_name: Option<String>,
    pub model_namespace: Option<i32>,
    pub page: Option<Page>,
}

crate::new_for! {
    #[pyo3(signature = (model_id = None, model_name = None, model_namespace = None, page = None) )]
    NamespaceModelsVariables,
    model_id: Option<i32>,
    model_name: Option<String>,
    model_namespace: Option<i32>,
    page: Option<Page>,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "NamespaceModelsVariables")]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct NamespaceModels {
    #[arguments(id: $model_id, page: $page, name: $model_name, namespace: $model_namespace)]
    pub model: PaginatedModel,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(into_wasm_abi))]
pub struct PaginatedModel {
    pub page: CurrentPage,
    pub total_pages: i32,
    pub total_items: i32,
    pub data: Vec<Model>,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct Model {
    pub id: i32,
    pub name: String,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub created_at: DateTime,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub last_modified: DateTime,
    pub namespace_id: i32,
}

jsvalue! {
    Model,
    PaginatedModel,
    NamespaceModels
}
