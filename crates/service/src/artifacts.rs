use actix_web::{
    body::BoxBody,
    dev::Response,
    http::{
        header::{self, HeaderName, HeaderValue},
        StatusCode,
    },
};
use anyhow::Error;
use sea_orm::{ConnectionTrait, TransactionError, TransactionTrait};
use std::{
    io::{Read, Seek, SeekFrom},
    pin::Pin,
    str::FromStr,
};

use bytes::Bytes;
use flymodel::{errs::FlymodelError, storage::StorageProvider};
use flymodel_entities::entities::{
    self,
    enums::{ArchiveEncoding, ArchiveFormat},
};
use flymodel_registry::storage::StorageOrchestrator;
use futures_util::Future;
use sea_orm::{ActiveEnum, DatabaseTransaction, EntityTrait};
use serde::Deserialize;

pub mod experiments;
pub mod model_version;

#[macro_export]
macro_rules! params_for {
    ($name: ident, [$(($arg: ident: $typ: ty)), + $(,)?]) => {
        paste::paste! {
            #[derive(Debug, Deserialize)]
            pub struct [< Upload $name Args >] {
                $(
                    pub $arg: $typ,
                )*
                #[serde(flatten)]
                pub blob: entities::upload::UploadBlobRequestParams,
            }

            #[derive(Debug, MultipartForm)]
            pub struct [< Upload $name Artifact >] {
                pub artifact: form::json::Json<[< Upload $name Args >]>,
                #[multipart(rename = "file")]
                pub file: TempFile,
            }
        }
    };
}

pub(crate) async fn guarded_upload<
    T: Send + Sync,
    C: TransactionTrait + ConnectionTrait,
    F: for<'c> FnOnce(
            &'c DatabaseTransaction,
            Option<String>,
        )
            -> Pin<Box<dyn Future<Output = Result<T, FlymodelError>> + Send + 'c>>
        + Send,
>(
    sink: &Box<dyn StorageProvider + std::marker::Send + Sync>,
    bs: Bytes,
    db: &C,
    key: String,
    with_tx: F,
) -> Result<T, FlymodelError> {
    let upload_res = sink.put(key.clone(), bs).await?;
    let created = match (&db)
        .transaction(|tx| (with_tx)(tx.to_owned(), upload_res.clone()))
        .await
    {
        Ok(created) => created,
        Err(e) => {
            let res = upload_res.clone();
            sink.del(key, res).await?;
            return Err(match e {
                TransactionError::Connection(conn) => FlymodelError::DbOperationError(conn),
                TransactionError::Transaction(tx) => tx,
            });
        }
    };
    Ok(created)
}

pub(crate) fn read_bts(
    mut bs: actix_multipart::form::tempfile::TempFile,
) -> Result<Bytes, FlymodelError> {
    let on_err = |err| FlymodelError::UploadError(Error::new(err));
    let mut arr = vec![];
    bs.file.seek(SeekFrom::Start(0)).map_err(on_err)?;
    bs.file.read_to_end(&mut arr).map_err(on_err)?;
    let bs = Bytes::from_iter(arr);
    if bs.len() == 0 {
        return Err(
            FlymodelError::NonDeterministicError("Uploads must contain data".into()).into(),
        );
    }
    Ok(bs)
}

#[derive(Deserialize, Debug)]
pub(crate) struct DownloadParams {
    artifact_id: i64,
}

pub(crate) async fn download_with_blob(
    blobref: &entities::object_blob::Model,
    bucket: &entities::bucket::Model,
    storage: &StorageOrchestrator,
    artifact_name: String,
) -> Result<Response<BoxBody>, FlymodelError> {
    let sink = storage
        .get(&bucket.name)
        .ok_or(FlymodelError::RuntimeDependencyError(format!(
            "missing {} bucket configurations",
            bucket.name
        )))?;

    let blob = sink
        .get(blobref.key.clone(), Some(blobref.version_id.clone()))
        .await?;

    let hash = sha256::digest(&*blob);
    if hash != blobref.sha256 {
        return Err((FlymodelError::IntegrityError {
            kind: "artifact tampering".into(),
            expect: blobref.sha256.clone(),
            receive: hash,
        })
        .into());
    }

    let mut resp = Response::new(StatusCode::OK).set_body(BoxBody::new(blob));

    let headers = resp.headers_mut();

    if let Some(content_typ) = blobref.encode {
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(match content_typ {
                ArchiveEncoding::Feather => "application/arrow",
                ArchiveEncoding::Json => "application/json",
                ArchiveEncoding::Parquet => "application/parquet",
            }),
        );
    }

    if let Some(encoding) = blobref.archive {
        headers.insert(
            header::CONTENT_ENCODING,
            HeaderValue::from_static(match encoding {
                ArchiveFormat::Gzip => "gzip",
                ArchiveFormat::Snappy => "snappy",
                ArchiveFormat::Tar => "tar",
                ArchiveFormat::Tzg => "tar, gzip",
                ArchiveFormat::Zip => "zip",
            }),
        );
    }

    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            r#"attachment; filename="{name}""#,
            name = artifact_name
        ))
        .map_err(FlymodelError::internal_error)?,
    );

    headers.insert(
        HeaderName::from_str("Digest").map_err(FlymodelError::internal_error)?,
        HeaderValue::from_str(&format!("sha256={hash}")).map_err(FlymodelError::internal_error)?,
    );

    Ok(resp)
}
