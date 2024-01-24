use crate::schema;
use serde::{Deserialize, Serialize};

#[derive(tsify::Tsify, cynic::InputObject, Clone, Debug, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct Page {
    pub size: i32,
    pub page: i32,
}

#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CurrentPage {
    pub size: i32,
    pub page: i32,
}
