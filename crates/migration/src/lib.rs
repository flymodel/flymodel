use std::sync::Mutex;

use hooks::Fixtures;
use sea_orm_migration::prelude::*;

pub mod cli;
pub mod hooks;
mod m000001_create_table;

static ONCE: std::sync::Once = std::sync::Once::new();
pub(crate) static FIXTURES: Mutex<Option<Fixtures>> = Mutex::new(None);

pub struct Migrator;

impl Migrator {
    pub fn init(fixtures: Option<Fixtures>) {
        ONCE.call_once(|| {
            if let Some(fixture) = fixtures {
                FIXTURES.lock().expect("ok mutex").replace(fixture);
            }
        });
    }
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m000001_create_table::Migration)]
    }
}
