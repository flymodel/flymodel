use flymodel_macros::{hybrid_feature_class, HybridEnum};

use crate::schema;

#[hybrid_feature_class(python = true)]
#[derive(HybridEnum, cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub enum Lifecycle {
    Prod,
    Qa,
    Stage,
    Test,
}

#[hybrid_feature_class(python = true)]
#[derive(HybridEnum, cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub enum ArchiveCompression {
    Gzip,
    #[cynic(rename = "LZ_4")]
    Lz4,
    Snappy,
    Tar,
    Tzg,
    Uncompressed,
    Zip,
    Zstd,
}

#[hybrid_feature_class(python = true)]
#[derive(HybridEnum, cynic::Enum, Clone, Copy, Debug)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub enum ArchiveFormat {
    Arrow,
    Csv,
    Html,
    Jpeg,
    Json,
    Jsonl,
    Md,
    Mov,
    #[cynic(rename = "MP_4")]
    Mp4,
    Msgpack,
    Parquet,
    Pdf,
    Png,
    Txt,
    Wav,
    Xls,
    Xml,
}
