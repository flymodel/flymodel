use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct DeleteModelVersionVariables {
    pub hard: Option<bool>,
    pub id: i32,
}

crate::new_for! {
    #[pyo3(signature = (id, hard = Some(false)))]
    DeleteModelVersionVariables,
    id: i32,
    hard: Option<bool>
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "DeleteModelVersionVariables")]
pub struct DeleteModelVersion {
    #[arguments(id: $id, hard: $hard)]
    pub delete_model_version: bool,
}
jsvalue! {
    DeleteModelVersion
}
