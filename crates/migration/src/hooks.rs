use flymodel_entities::{entities, prelude::*};
use sea_orm::{Database, DbConn, DbErr};
use sea_orm_migration::{
    sea_orm::{entity::*, query::*},
    SchemaManagerConnection,
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FixtureData {
    pub namespaces: Vec<entities::namespace::Model>,
    pub buckets: Vec<entities::bucket::Model>,
}

pub struct Fixtures;

impl Fixtures {
    pub const fn basic() -> &'static str {
        include_str!("../seeds/basic.yaml")
    }

    pub fn load(base: &'static str) -> FixtureData {
        serde_yaml::from_str(base).unwrap()
    }

    pub async fn insert_models(
        conn: &'_ SchemaManagerConnection<'_>,
        fixture: FixtureData,
    ) -> Result<(), DbErr> {
        let act: Vec<_> = fixture
            .namespaces
            .iter()
            .map(|th| th.clone().into_active_model())
            .collect();
        Namespace::insert_many(act).exec(conn).await?;

        let act: Vec<_> = fixture
            .buckets
            .iter()
            .map(|th| th.clone().into_active_model())
            .collect();

        Bucket::insert_many(act).exec(conn).await?;
        Ok(())
    }
}

mod test {

    #[test]
    fn test() {
        use super::Fixtures;
        println!("loaded: {:#?}", Fixtures::load(Fixtures::basic()));
    }
}
