use crate::{jsvalue, scalars::DateTime, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct UpdateModelVariables {
    pub id: i32,
    pub name: String,
}
crate::new_for! {
    UpdateModelVariables,
    id: i32,
    name: &str,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "UpdateModelVariables")]
pub struct UpdateModel {
    #[arguments(id: $id, name: $name)]
    pub update_model: Model,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub last_modified: DateTime,
}
jsvalue! {
    Model,
    UpdateModel
}
