use sea_orm_migration::prelude::*;

use crate::FIXTURES;

#[derive(DeriveMigrationName)]
pub struct Migration;

static UP: &str = include_str!("../sql/pg/000001_up.sql");
static DOWN: &str = include_str!("../sql/pg/000001_down.sql");

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(UP).await?;
        let fixture = FIXTURES.lock().expect("ok mutex").clone();
        if let Some(fixture) = fixture {
            fixture.insert_models(manager.get_connection()).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(DOWN).await?;
        Ok(())
    }
}
