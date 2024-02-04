use crate::{enums::*, jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct UpdateModelVersionStateVariables {
    pub id: i32,
    pub state: Lifecycle,
}

crate::new_for! {
    UpdateModelVersionStateVariables,
    id: i32,
    state: Lifecycle
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(
    graphql_type = "Mutation",
    variables = "UpdateModelVersionStateVariables"
)]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
pub struct UpdateModelVersionState {
    #[arguments(id: $id, state: $state)]
    pub update_model_version_state: ModelState,
}

#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
pub struct ModelState {
    pub id: i32,
    pub version_id: i32,
    pub state: Lifecycle,
}

jsvalue! {
    ModelState,
    UpdateModelVersionState
}
