use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlymodelError {
    #[error("Failed to connect to database")]
    DbConnectionError(#[from] DbErr),
}

pub type FlymodelResult<T> = Result<T, FlymodelError>;
