use sea_orm_migration::prelude::*;

pub mod hooks;
mod m000001_create_table;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m000001_create_table::Migration)]
    }
}
