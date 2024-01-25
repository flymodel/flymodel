use flymodel_macros::hybrid_feature_class;

use crate::schema;

#[hybrid_feature_class("python")]
#[derive(tsify::Tsify, cynic::Enum, Clone, Copy, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Lifecycle {
    #[cynic(rename = "prod")]
    Prod,
    #[cynic(rename = "qa")]
    Qa,
    #[cynic(rename = "stage")]
    Stage,
    #[cynic(rename = "test")]
    Test,
}
