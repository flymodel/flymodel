use async_graphql::ErrorExtensions;
use aws_sdk_s3::operation::{get_object::GetObjectError, put_object::PutObjectError};
use aws_smithy_runtime_api::{client::result::SdkError as AwsError, http::Response as AwsResponse};
use sea_orm::DbErr;
use std::{str::FromStr, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlymodelError {
    #[error("Failed to connect to database")]
    DbConnectionError(DbErr),

    #[error("Database setup error: {0}")]
    DbSetupError(DbErr),

    #[error("Database operation error: {0}")]
    DbOperationError(#[from] DbErr),

    #[error("Database operation error: {0}")]
    DbLoaderError(#[from] Arc<DbErr>),

    #[error("Invalid permission: {0}")]
    InvalidPermission(String),

    #[error("Invalid ID: {0}")]
    IdParsingError(<i64 as FromStr>::Err),

    #[error("Integrity error ({kind}): expected {expect}, but received {receive}")]
    IntegrityError {
        kind: String,
        expect: String,
        receive: String,
    },

    #[error("Storage setup error: {0}")]
    StorageSetupError(anyhow::Error),

    #[error("S3 operation error (put): {0}")]
    S3PutObjectError(#[from] AwsError<PutObjectError, AwsResponse>),

    #[error("S3 operation error (get): {0}")]
    S3GetObjectError(#[from] AwsError<GetObjectError, AwsResponse>),

    #[error("S3 operation error (get::collect()): {0}")]
    S3BinaryLoadError(#[from] aws_sdk_s3::primitives::ByteStreamError),

    #[error("Missing runtime dependency: {0}")]
    RuntimeDependencyError(String),
}

impl FlymodelError {
    pub fn code(&self) -> u64 {
        (match self {
            Self::DbConnectionError(_) => 1,
            Self::DbSetupError(_) => 2,
            Self::DbOperationError(_) => 3,
            Self::DbLoaderError(_) => 4,
            Self::IdParsingError(_) => 5,
            Self::InvalidPermission(_) => 6,
            Self::IntegrityError { .. } => 7,
            Self::StorageSetupError(_) => 8,
            Self::S3GetObjectError(_) => 9,
            Self::S3PutObjectError(_) => 10,
            Self::S3BinaryLoadError(_) => 11,
            Self::RuntimeDependencyError(_) => 12,
        } + 9008)
    }

    pub fn code_str(&self) -> &'static str {
        match self {
            Self::DbConnectionError(..)
            | Self::DbSetupError(..)
            | Self::DbLoaderError(..)
            | Self::DbOperationError(..) => "DatabaseError",
            Self::IdParsingError(..) => "IdParsingError",
            Self::S3BinaryLoadError(..)
            | Self::S3GetObjectError(..)
            | Self::S3PutObjectError(..) => "StorageError",
            Self::IntegrityError { .. } => "IntegrityError",
            Self::InvalidPermission(..) => "InvalidPermission",
            _ => "SystemError",
        }
    }

    pub fn code_description(&self) -> String {
        match self {
            Self::DbConnectionError(..)
            | Self::DbSetupError(..)
            | Self::DbLoaderError(..)
            | Self::DbOperationError(..) => "A database error has occured",
            Self::IdParsingError(..) => "An error occured parsing an entity ID",
            Self::IntegrityError { .. } => "An integrity error has occured",
            Self::InvalidPermission(..) => "An invalid permission has been provided",
            Self::S3BinaryLoadError(..) => "An error occured loading binary data from storage",
            Self::S3GetObjectError(..) => "An error occured loading data from storage",
            Self::S3PutObjectError(..) => "An error occured uploading data to storage",
            _ => "A system error occured",
        }
        .to_string()
    }

    pub fn into_graphql_error(&self) -> async_graphql::Error {
        tracing::error!("an error occured serving graphql: {}", self);
        async_graphql::Error::new(self.code_description()).extend_with(|_, ext| {
            ext.set("code", self.code());
            ext.set("kind", self.code_str());
        })
    }
}

pub type FlymodelResult<T> = Result<T, FlymodelError>;
