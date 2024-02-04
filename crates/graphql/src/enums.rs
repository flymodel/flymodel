use flymodel_macros::{hybrid_feature_class, HybridEnum};

use crate::schema;

#[derive(HybridEnum, cynic::Enum, Clone, Copy, Debug)]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
pub enum Lifecycle {
    Prod,
    Qa,
    Stage,
    Test,
}

#[derive(HybridEnum, cynic::Enum, Clone, Copy, Debug)]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
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

#[derive(HybridEnum, cynic::Enum, Clone, Copy, Debug)]
#[hybrid_feature_class(python = true, ts = true, rename_ts = true)]
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
