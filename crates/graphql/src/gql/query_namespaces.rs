use crate::{
    scalars::DateTime,
    schema::{self},
};

use crate::{fragments::*, jsvalue};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct QueryNamespacesVariables {
    pub page: Option<Page>,
}

crate::new_for! {
    #[pyo3(signature = (page = None) )]
    QueryNamespacesVariables,
    page: Option<Page>,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "QueryNamespacesVariables")]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
pub struct QueryNamespaces {
    #[arguments(page: $page)]
    pub namespace: PaginatedNamespace,
}

#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
pub struct PaginatedNamespace {
    pub page: CurrentPage,
    pub total_pages: i32,
    pub total_items: i32,
    pub data: Vec<Namespace>,
}

#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]

pub struct Namespace {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub created_at: DateTime,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub last_modified: DateTime,
}

jsvalue! {
    Namespace,
    PaginatedNamespace,
    QueryNamespaces
}
