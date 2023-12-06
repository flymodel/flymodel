use std::str::FromStr;

use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlymodelError {
    #[error("Failed to connect to database")]
    DbConnectionError(#[from] DbErr),

    #[error("Invalid permission: {0}")]
    InvalidPermission(String),

    #[error("Invalid ID: {0}")]
    IdParsingError(<i64 as FromStr>::Err),
}

pub type FlymodelResult<T> = Result<T, FlymodelError>;
