use crate::{jsvalue, scalars::*, schema};
use flymodel_macros::hybrid_feature_class;

use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct CreateNamespaceVariables {
    pub name: String,
    pub description: String,
}

crate::new_for! {
    CreateNamespaceVariables,
    name: &str,
    description: &str,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateNamespaceVariables")]
pub struct CreateNamespace {
    #[arguments(name: $name, description: $description)]
    pub create_namespace: Namespace,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
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
    CreateNamespace
}
