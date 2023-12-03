use sea_orm::entity::prelude::*;

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "u32", db_type = "Integer", enum_name = "storage_role")]
#[serde(rename_all = "lowercase")]
pub enum StorageRole {
    #[sea_orm(num_value = 0)]
    Test,
    #[sea_orm(num_value = 1)]
    Qa,
    #[sea_orm(num_value = 2)]
    Staging,
    #[sea_orm(num_value = 3)]
    Prod,
}

impl ToString for StorageRole {
    fn to_string(&self) -> String {
        match self {
            Self::Test => "test",
            Self::Qa => "qa",
            Self::Staging => "staging",
            Self::Prod => "prod",
        }
        .into()
    }
}

impl Default for StorageRole {
    fn default() -> Self {
        Self::Test
    }
}
