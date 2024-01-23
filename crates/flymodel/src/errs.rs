use actix_web::{
    body::BoxBody,
    http::{
        header::{self, HeaderValue},
        StatusCode,
    },
    HttpResponse,
};
use async_graphql::ErrorExtensions;
use aws_sdk_s3::operation::{get_object::GetObjectError, put_object::PutObjectError};
use aws_smithy_runtime_api::{client::result::SdkError as AwsError, http::Response as AwsResponse};
use sea_orm::DbErr;
use std::{error::Error, str::FromStr, sync::Arc};
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

    #[error("Constraint error: {0}")]
    ContraintError(String),

    #[error("Upload error: {0}")]
    UploadError(anyhow::Error),

    #[error("Non-Deterministic behaviour error: {0}")]
    NonDeterministicError(String),

    #[error("Invalid resource ID: {0}")]
    InvalidResourceId(i64),

    #[error("Internal Error: {0}")]
    InternalError(anyhow::Error),
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
            Self::ContraintError(_) => 13,
            Self::UploadError(_) => 14,
            Self::NonDeterministicError(_) => 15,
            Self::InvalidResourceId(_) => 16,
            Self::InternalError(_) => 17,
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
            Self::ContraintError(..) => "ContraintError",
            Self::UploadError(..) => "UploadError",
            Self::NonDeterministicError(..) => "NonDeterministicBehaviourError",
            Self::InvalidResourceId(..) => "InvalidResourceId",
            _ => "SystemError",
        }
    }

    pub fn code_description(&self) -> String {
        match self {
            Self::DbConnectionError(..)
            | Self::DbSetupError(..)
            | Self::DbLoaderError(..)
            | Self::DbOperationError(..) => "A database error has occured".to_string(),
            Self::IdParsingError(..) => "An error occured parsing an entity ID".to_string(),
            Self::IntegrityError { .. } => "An integrity error has occured".to_string(),
            Self::InvalidPermission(..) => "An invalid permission has been provided".to_string(),
            Self::S3BinaryLoadError(..) => {
                "An error occured loading binary data from storage".to_string()
            }
            Self::S3GetObjectError(..) => "An error occured loading data from storage".to_string(),
            Self::S3PutObjectError(..) => "An error occured uploading data to storage".to_string(),
            Self::ContraintError(source) => {
                format!("The following contraint failed validation: {source}")
            }
            Self::UploadError(..) => {
                format!("A system error occured while uploading data")
            }
            Self::NonDeterministicError(..) => {
                format!("Non deterministic behaviour was detected")
            }
            Self::InvalidResourceId(id) => {
                format!("{id} could not be found")
            }
            _ => "A system error occured".to_string(),
        }
    }

    pub fn into_graphql_error(&self) -> async_graphql::Error {
        tracing::error!("an error occured serving graphql: {}", self);
        async_graphql::Error::new(self.code_description()).extend_with(|_, ext| {
            ext.set("code", self.code());
            ext.set("kind", self.code_str());
        })
    }

    pub fn internal_error<'a, E: Error + Sync + Send + 'static>(err: E) -> FlymodelError {
        FlymodelError::InternalError(anyhow::Error::from(err))
    }
}

impl actix_web::error::ResponseError for FlymodelError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::IdParsingError(..) => StatusCode::BAD_REQUEST,
            Self::IntegrityError { .. } | Self::ContraintError(..) => {
                StatusCode::EXPECTATION_FAILED
            }
            Self::InvalidPermission(..) => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        tracing::error!("an error occured serving an api: {}", self);

        let mut resp = HttpResponse::new(self.status_code()).set_body(BoxBody::new(
            // this is infallible serialization
            match serde_json::to_vec(&serde_json::json!({
                "code": self.code(),
                "kind": self.code_str()
            })) {
                Ok(enc) => enc,
                Err(..) => unreachable!(),
            },
        ));

        let headers = resp.headers_mut();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        resp
    }
}

pub type FlymodelResult<T> = Result<T, FlymodelError>;
