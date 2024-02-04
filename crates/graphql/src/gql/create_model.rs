use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct CreateModelVariables {
    pub name: String,
    pub namespace: i32,
}

crate::new_for! {
    CreateModelVariables,
    name: &str,
    namespace: i32,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateModelVariables")]
pub struct CreateModel {
    #[arguments(namespace: $namespace, name: $name)]
    pub create_model: Model,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub namespace_id: i32,
}

jsvalue! {
    Model,
    CreateModel
}
