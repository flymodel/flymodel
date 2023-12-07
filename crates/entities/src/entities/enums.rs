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
    #[sea_orm(string_value = "gzip")]
    Gzip,
    #[sea_orm(string_value = "snappy")]
    Snappy,
    #[sea_orm(string_value = "tar")]
    Tar,
    #[sea_orm(string_value = "tzg")]
    Tzg,
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
    #[sea_orm(string_value = "feather")]
    Feather,
    #[sea_orm(string_value = "json")]
    Json,
    #[sea_orm(string_value = "parquet")]
    Parquet,
}
