use crate::{jsvalue, scalars::DateTime, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct UpdateNamespaceVariables {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}

crate::new_for! {
    #[pyo3(signature = (id, name = None, description = None))]
    UpdateNamespaceVariables,
    id: i32,
    name: Option<String>,
    description: Option<String>
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "UpdateNamespaceVariables")]

pub struct UpdateNamespace {
    #[arguments(id: $id, name: $name, description: $description)]
    pub update_namespace: Namespace,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
pub struct Namespace {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub last_modified: DateTime,
}
jsvalue! {
    Namespace,
    UpdateNamespace
}
