use async_graphql::dataloader::DataLoader;
use sea_orm::DatabaseConnection;

pub struct OrmDataloader {
    pub db: DatabaseConnection,
}

pub type Database = DataLoader<OrmDataloader>;

impl OrmDataloader {
    pub fn new(db: DatabaseConnection) -> Database {
        Database::new(Self { db }, tokio::spawn)
    }
}
