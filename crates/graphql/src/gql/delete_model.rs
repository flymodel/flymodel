use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct DeleteModelVariables {
    pub id: i32,
}

crate::new_for! {
    DeleteModelVariables,
    id: i32,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "DeleteModelVariables")]
pub struct DeleteModel {
    #[arguments(id: $id)]
    pub delete_model: bool,
}
jsvalue! {
    DeleteModel
}
