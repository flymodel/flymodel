use flymodel_entities::{entities, prelude::*};
use sea_orm::DbErr;
use sea_orm_migration::{sea_orm::entity::*, SchemaManagerConnection};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FixtureData {
    #[serde(default = "Vec::default")]
    pub namespaces: Vec<entities::namespace::Model>,
    #[serde(default = "Vec::default")]
    pub buckets: Vec<entities::bucket::Model>,
    #[serde(default = "Vec::default")]
    pub models: Vec<entities::model::Model>,
    #[serde(default = "Vec::default")]
    pub versions: Vec<entities::model_version::Model>,
    #[serde(default = "Vec::default")]
    pub states: Vec<entities::model_state::Model>,
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

    async fn maybe_insert<E, A, AM>(
        src: Vec<A>,
        conn: &'_ SchemaManagerConnection<'_>,
        with_am: impl Fn(&mut AM),
    ) -> Result<(), DbErr>
    where
        E: EntityTrait + Sized,
        A: IntoActiveModel<AM> + Clone + Sized,
        AM: ActiveModelTrait<Entity = E> + Sized,
    {
        if src.len() == 0 {
            return Ok(());
        }
        E::insert_many(
            src.iter()
                .map(|th| {
                    let mut model = th.clone().into_active_model();
                    with_am(&mut model);
                    model
                })
                .collect::<Vec<_>>(),
        )
        .exec(conn)
        .await?;
        Ok(())
    }

    pub async fn insert_models(&self, conn: &'_ SchemaManagerConnection<'_>) -> Result<(), DbErr> {
        let fixture = self.load();
        let act: Vec<_> = fixture
            .namespaces
            .iter()
            .map(|th| th.clone().into_active_model())
            .collect();

        Self::maybe_insert::<Namespace, _, _>(act, conn, |am| {
            am.id = ActiveValue::NotSet;
        })
        .await?;
        Self::maybe_insert::<Bucket, _, _>(fixture.buckets, conn, |am| {
            am.id = ActiveValue::NotSet;
        })
        .await?;
        Self::maybe_insert::<Model, _, _>(fixture.models, conn, |am| {
            am.id = ActiveValue::NotSet;
        })
        .await?;
        Self::maybe_insert::<ModelVersion, _, _>(fixture.versions, conn, |am| {
            am.id = ActiveValue::NotSet;
        })
        .await?;
        Self::maybe_insert::<ModelState, _, _>(fixture.states, conn, |am| {
            am.id = ActiveValue::NotSet;
        })
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
