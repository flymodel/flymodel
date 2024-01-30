use crate::schema;
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(cynic::InputObject, Clone, Debug, Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(from_wasm_abi))]
pub struct Page {
    pub size: i32,
    pub page: i32,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(into_wasm_abi))]
pub struct CurrentPage {
    pub size: i32,
    pub page: i32,
}

#[cfg_attr(feature = "python", pyo3::pymethods)]
impl CurrentPage {
    pub fn next(&self) -> Self {
        Self {
            size: self.size,
            page: self.page + 1,
        }
    }

    pub fn prev(&self) -> Self {
        if self.page > 0 {
            Self {
                size: self.size,
                page: self.page - 1,
            }
        } else {
            self.clone()
        }
    }
}

#[cfg_attr(feature = "python", pyo3::pymethods)]
impl Page {
    #[new]
    pub fn new(size: i32, page: i32) -> Self {
        Self { size, page }
    }
}
