use crate::{enums::*, jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(from_wasm_abi))]
pub struct UpdateModelVersionStateVariables {
    pub id: i32,
    pub state: Lifecycle,
}

crate::new_for! {
    UpdateModelVersionStateVariables,
    id: i32,
    state: Lifecycle
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(
    graphql_type = "Mutation",
    variables = "UpdateModelVersionStateVariables"
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct UpdateModelVersionState {
    #[arguments(id: $id, state: $state)]
    pub update_model_version_state: ModelState,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(into_wasm_abi))]
pub struct ModelState {
    pub id: i32,
    pub version_id: i32,
    pub state: Lifecycle,
}

jsvalue! {
    ModelState,
    UpdateModelVersionState
}
