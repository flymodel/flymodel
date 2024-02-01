use async_graphql::Enum;
use sea_orm::entity::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Enum,
    EnumIter,
    DeriveActiveEnum,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[graphql(name = "ArchiveCompression")]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "archive_compression"
)]
pub enum ArchiveCompression {
    #[sea_orm(string_value = "gzip")]
    Gzip,
    #[sea_orm(string_value = "lz4")]
    Lz4,
    #[sea_orm(string_value = "snappy")]
    Snappy,
    #[sea_orm(string_value = "tar")]
    Tar,
    #[sea_orm(string_value = "tzg")]
    Tzg,
    #[sea_orm(string_value = "uncompressed")]
    Uncompressed,
    #[sea_orm(string_value = "zip")]
    Zip,
    #[sea_orm(string_value = "zstd")]
    Zstd,
}

#[derive(
    Copy,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Enum,
    EnumIter,
    DeriveActiveEnum,
    serde::Serialize,
    serde::Deserialize,
)]
#[graphql(name = "ArchiveFormat")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "archive_format")]
pub enum ArchiveFormat {
    #[sea_orm(string_value = "arrow")]
    Arrow,
    #[sea_orm(string_value = "csv")]
    Csv,
    #[sea_orm(string_value = "html")]
    Html,
    #[sea_orm(string_value = "jpeg")]
    Jpeg,
    #[sea_orm(string_value = "json")]
    Json,
    #[sea_orm(string_value = "jsonl")]
    Jsonl,
    #[sea_orm(string_value = "md")]
    Md,
    #[sea_orm(string_value = "mov")]
    Mov,
    #[sea_orm(string_value = "mp4")]
    Mp4,
    #[sea_orm(string_value = "msgpack")]
    Msgpack,
    #[sea_orm(string_value = "parquet")]
    Parquet,
    #[sea_orm(string_value = "pdf")]
    Pdf,
    #[sea_orm(string_value = "png")]
    Png,
    #[sea_orm(string_value = "txt")]
    Txt,
    #[sea_orm(string_value = "wav")]
    Wav,
    #[sea_orm(string_value = "xls")]
    Xls,
    #[sea_orm(string_value = "xml")]
    Xml,
}
