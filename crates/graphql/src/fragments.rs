use crate::schema;
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class("python")]
#[derive(tsify::Tsify, cynic::InputObject, Clone, Debug, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct Page {
    pub size: i32,
    pub page: i32,
}

#[hybrid_feature_class("python")]
#[derive(tsify::Tsify, cynic::QueryFragment, Clone, Debug, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CurrentPage {
    pub size: i32,
    pub page: i32,
}

#[cfg(feature = "python")]
#[pyo3::prelude::pymethods]
impl CurrentPage {
    #[new]
    pub fn new(size: i32, page: i32) -> Self {
        Self { size, page }
    }
}
