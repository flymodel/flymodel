use async_graphql::Enum;
use sea_orm::entity::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Copy,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Enum,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
#[graphql(name = "Lifecycle")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "lifecycle")]
pub enum Lifecycle {
    #[sea_orm(string_value = "prod")]
    #[graphql(name = "prod")]
    Prod,
    #[sea_orm(string_value = "qa")]
    #[graphql(name = "qa")]
    Qa,
    #[sea_orm(string_value = "stage")]
    #[graphql(name = "stage")]
    Stage,
    #[sea_orm(string_value = "test")]
    #[graphql(name = "test")]
    Test,
}

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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "archival_format")]
pub enum ArchivalFormat {
    #[sea_orm(string_value = "tgz")]
    Tgz,
    #[sea_orm(string_value = "zip")]
    Zip,
}
