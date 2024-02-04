use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct CreateModelVersionVariables {
    pub model_id: i32,
    pub version_tag: String,
}

crate::new_for! {
    CreateModelVersionVariables,
    version_tag: &str,
    model_id: i32,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateModelVersionVariables")]
pub struct CreateModelVersion {
    #[arguments(model: $model_id, name: $version_tag)]
    pub create_model_version: ModelVersion,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
pub struct ModelVersion {
    pub id: i32,
    pub version: String,
    pub model_id: i32,
}

jsvalue! {
    ModelVersion,
    CreateModelVersion
}
