use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use partial_context::PartialContext;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize, PartialContext)]
#[context_needs(
    #[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)],
    #[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
)]
pub struct DeleteNamespaceVariables {
    #[context]
    pub id: i32,
}

crate::new_for! {
    DeleteNamespaceVariables,
    id: i32,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "DeleteNamespaceVariables")]
pub struct DeleteNamespace {
    #[arguments(id: $id)]
    pub delete_namespace: bool,
}

jsvalue! {
    DeleteNamespace
}
