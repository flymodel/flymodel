use std::fmt::Display;

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
    PartialOrd,
)]
#[serde(rename_all = "lowercase")]
#[graphql(name = "Lifecycle")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "lifecycle")]
pub enum Lifecycle {
    #[sea_orm(string_value = "test")]
    #[serde(rename = "test")]
    Test = 0,

    #[sea_orm(string_value = "qa")]
    #[serde(rename = "qa")]
    Qa = 1,

    #[sea_orm(string_value = "stage")]
    #[serde(rename = "stage")]
    Stage = 2,

    #[sea_orm(string_value = "prod")]
    #[serde(rename = "prod")]
    Prod = 3,
}

impl Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Prod => "prod",
                Self::Qa => "qa",
                Self::Stage => "stage",
                Self::Test => "test",
            }
        )
    }
}
