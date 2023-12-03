use sea_orm_migration::prelude::*;

use crate::hooks::Fixtures;

#[derive(DeriveMigrationName)]
pub struct Migration;

static UP: &str = include_str!("../sql/pg/000001_up.sql");
static DOWN: &str = include_str!("../sql/pg/000001_down.sql");

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(UP).await?;
        if std::env::var("TEST_DATA").is_ok() {
            Fixtures::insert_models(manager.get_connection(), Fixtures::load(Fixtures::basic()))
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(DOWN).await?;
        Ok(())
    }
}
