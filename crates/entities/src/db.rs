use std::marker::PhantomData;

use async_graphql::dataloader::DataLoader;
use sea_orm::DatabaseConnection;

pub struct DbLoader<T> {
    pub db: DatabaseConnection,
    ph: PhantomData<T>,
}

pub type Database<T> = DataLoader<DbLoader<T>>;

impl<T> DbLoader<T> {
    pub fn new(db: DatabaseConnection) -> Database<T> {
        Database::new(
            Self {
                db,
                ph: PhantomData,
            },
            tokio::spawn,
        )
    }
}
