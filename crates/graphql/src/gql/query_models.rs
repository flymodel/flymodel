use crate::{fragments::*, jsvalue, scalars::*, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
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

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "NamespaceModelsVariables")]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
pub struct NamespaceModels {
    #[arguments(id: $model_id, page: $page, name: $model_name, namespace: $model_namespace)]
    pub model: PaginatedModel,
}

#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
pub struct PaginatedModel {
    pub page: CurrentPage,
    pub total_pages: i32,
    pub total_items: i32,
    pub data: Vec<Model>,
}

#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
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
