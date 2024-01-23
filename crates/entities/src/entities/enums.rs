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
#[graphql(name = "ArchivalFormat")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "archive_format")]
pub enum ArchiveFormat {
    #[serde(rename = "gzip")]
    #[sea_orm(string_value = "gzip")]
    Gzip,
    #[serde(rename = "snappy")]
    #[sea_orm(string_value = "snappy")]
    Snappy,
    #[serde(rename = "tar")]
    #[sea_orm(string_value = "tar")]
    Tar,
    #[serde(rename = "tzg")]
    #[sea_orm(string_value = "tzg")]
    Tzg,
    #[serde(rename = "zip")]
    #[sea_orm(string_value = "zip")]
    Zip,
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
#[graphql(name = "ArchiveEncoding")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "archive_encoding")]
pub enum ArchiveEncoding {
    #[serde(rename = "feather")]
    #[sea_orm(string_value = "feather")]
    Feather,
    #[serde(rename = "json")]
    #[sea_orm(string_value = "json")]
    Json,
    #[serde(rename = "parquet")]
    #[sea_orm(string_value = "parquet")]
    Parquet,
}
