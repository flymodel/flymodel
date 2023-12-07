use async_graphql::{dataloader::DataLoader, Context};
use flymodel::errs::FlymodelError;
use flymodel_tracing::tracer::OtlpTracerConfig;

use sea_orm::{query::Select, DatabaseConnection, EntityTrait};
use std::marker::PhantomData;
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

    pub fn with_context<'ctx>(ctx: &Context<'ctx>) -> Result<&'ctx Database<T>, FlymodelError> {
        ctx.data::<Database<T>>().map_err(|err| {
            trace!("an actual error is being suppressed: {:#?}", err);
            let t = std::any::type_name::<T>();
            warn!("missing dependency at runtime: Database<{}>", t);
            // SAFETY: threads
            FlymodelError::RuntimeDependencyError(t.to_string())
        })
    }
}

pub type QueryResult<T> = Result<T, async_graphql::Error>;

#[inline]
pub async fn handle_all<T>(
    db: DbLoader<T>,
    sel: Select<T>,
) -> QueryResult<Vec<<T as EntityTrait>::Model>>
where
    T: EntityTrait,
{
    sel.all(&db.db)
        .await
        .map_err(|err| FlymodelError::DbOperationError(err).into_graphql_error())
}

#[inline]
pub async fn handle_one<T>(
    db: DbLoader<T>,
    sel: Select<T>,
) -> QueryResult<Option<<T as EntityTrait>::Model>>
where
    T: EntityTrait,
{
    sel.one(&db.db)
        .await
        .map_err(|err| FlymodelError::DbOperationError(err).into_graphql_error())
}
