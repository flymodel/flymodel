use flymodel_entities::{entities, prelude::*};
use sea_orm::DbErr;
use sea_orm_migration::{sea_orm::entity::*, SchemaManagerConnection};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FixtureData {
    pub namespaces: Vec<entities::namespace::Model>,
    pub buckets: Vec<entities::bucket::Model>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Fixtures {
    Basic,
    MultiRegion,
}

impl Fixtures {
    pub const fn fixture(&self) -> &'static str {
        match self {
            Self::Basic => include_str!("../seeds/basic.yaml"),
            Self::MultiRegion => include_str!("../seeds/multi-region.yaml"),
        }
    }

    pub fn load(&self) -> FixtureData {
        serde_yaml::from_str(self.fixture()).unwrap()
    }

    pub async fn insert_models(&self, conn: &'_ SchemaManagerConnection<'_>) -> Result<(), DbErr> {
        let fixture = self.load();
        let act: Vec<_> = fixture
            .namespaces
            .iter()
            .map(|th| th.clone().into_active_model())
            .collect();
        Namespace::insert_many(act).exec(conn).await?;
        Bucket::insert_many(
            fixture
                .buckets
                .iter()
                .map(|th| th.clone().into_active_model())
                .collect::<Vec<_>>(),
        )
        .exec(conn)
        .await?;
        Ok(())
    }
}

mod test {

    #[test]
    fn test_basic() {
        use super::Fixtures;
        println!("loaded: {:#?}", Fixtures::Basic.load());
    }

    #[test]
    fn test_multi_region() {
        use super::Fixtures;
        println!("loaded: {:#?}", Fixtures::MultiRegion.load());
    }
}
