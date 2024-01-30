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
    #[cynic(rename = "PROD")]
    Prod,
    #[cynic(rename = "QA")]
    Qa,
    #[cynic(rename = "STAGE")]
    Stage,
    #[cynic(rename = "TEST")]
    Test,
}
