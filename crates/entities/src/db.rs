use async_graphql::{dataloader::DataLoader, Context};
use flymodel_tracing::tracer::OtlpTracerConfig;

use sea_orm::{DatabaseConnection, DbErr};
use std::{marker::PhantomData, sync::Arc};
use tracing::{trace, warn};

pub struct DbLoader<T> {
    pub db: DatabaseConnection,
    ph: PhantomData<T>,
}

pub type Database<T> = DataLoader<DbLoader<T>>;

impl<T> DbLoader<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(db: DatabaseConnection, _tracer: Option<OtlpTracerConfig>) -> Database<T> {
        Database::new(
            Self {
                db,
                ph: PhantomData,
            },
            tokio::spawn,
        )
    }

    pub fn with_context<'ctx>(ctx: &Context<'ctx>) -> Result<&'ctx Database<T>, Arc<DbErr>> {
        ctx.data::<Database<T>>().map_err(|err| {
            trace!("an actual error is being suppressed: {:#?}", err);
            warn!(
                "missing dependency at runtime: Database<{}>",
                std::any::type_name::<T>()
            );
            // SAFETY: threads
            Arc::new(DbErr::Custom("System Error".to_string()))
        })
    }
}

pub type QueryResult<T> = Result<T, Arc<DbErr>>;
