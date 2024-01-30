use flymodel_macros::hybrid_feature_class;

use crate::schema;

#[hybrid_feature_class(python = true)]
#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
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
