use std::{
    io::{Read, Seek, SeekFrom},
    str::FromStr,
    sync::Arc,
};

use actix_web::{
    body::BoxBody,
    dev::Response,
    http::{
        header::{self, HeaderName, HeaderValue},
        StatusCode,
    },
    routes,
    web::{self, Data, Query},
    Responder,
};
use anyhow::Error;
use async_graphql::dataloader::{DataLoader, Loader};
use bytes::Bytes;
use flymodel::errs::FlymodelError;
use flymodel_entities::{
    db::DbLoader,
    entities::{
        self,
        enums::{ArchiveEncoding, ArchiveFormat},
    },
};
use flymodel_registry::storage::StorageOrchestrator;
use sea_orm::{
    sea_query::{Alias, Expr},
    ActiveEnum, ColumnTrait, EntityTrait, QueryFilter,
};
use serde::Deserialize;
use tracing::{debug, info};

use actix_multipart::form::{self, tempfile::TempFile, MultipartForm};

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

params_for!(ModelVersion, [(model_version: i64)]);

#[routes]
#[post("/upload/model-version-artifact")]
pub async fn upload_model_version_artifact(
    MultipartForm(form): MultipartForm<UploadModelVersionArtifact>,
) -> String {
    // form.;

    info!("got {:#?}", form);
    "OK".into()
}

params_for!(Experiment, [(experiment: i64)]);

fn read_bts(mut bs: actix_multipart::form::tempfile::TempFile) -> Result<Bytes, FlymodelError> {
    let on_err = |err| FlymodelError::UploadError(Error::new(err));
    let mut arr = vec![];
    bs.file.seek(SeekFrom::Start(0)).map_err(on_err)?;
    bs.file.read_to_end(&mut arr).map_err(on_err)?;
    Ok(Bytes::from_iter(arr))
}

#[derive(Deserialize, Debug)]
struct DownloadParams {
    artifact_id: i64,
}

#[routes]
#[get("/download/experiment-artifact")]
pub async fn download_experiment_artifact(
    storage: Data<Arc<StorageOrchestrator>>,
    params: Query<DownloadParams>,
    artifact: Data<DataLoader<DbLoader<entities::experiment_artifact::Model>>>,
    experiment: Data<DataLoader<DbLoader<entities::experiment::Model>>>,
    namespaces: Data<DataLoader<DbLoader<entities::namespace::Model>>>,
    versions: Data<DataLoader<DbLoader<entities::model_version::Model>>>,
    buckets: Data<DataLoader<DbLoader<entities::bucket::Model>>>,
    blobs: Data<DataLoader<DbLoader<entities::object_blob::Model>>>,
) -> actix_web::Result<impl Responder> {
    let on_missing = || FlymodelError::InvalidResourceId(params.artifact_id);
    let on_err = |err| FlymodelError::DbLoaderError(err);

    let artifact = artifact
        .loader()
        .load(&[params.artifact_id])
        .await
        .map_err(on_err)?;

    let artifact = artifact.get(&params.artifact_id).ok_or_else(on_missing)?;

    let cte = get_common_from_experiment(
        artifact.experiment_id,
        experiment,
        namespaces,
        versions,
        buckets,
    )
    .await?;

    let blobref = blobs
        .loader()
        .load(&[artifact.blob])
        .await
        .map_err(on_err)?;

    let blobref = blobref.get(&artifact.blob).ok_or_else(on_missing)?;

    let sink = storage
        .get(&cte.bucket.name)
        .ok_or(FlymodelError::RuntimeDependencyError(format!(
            "missing {} bucket configurations",
            cte.bucket.name
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

    let body = BoxBody::new(blob);
    let mut resp = Response::new(StatusCode::OK).set_body(body);

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
            name = artifact.name
        ))
        .map_err(FlymodelError::internal_error)?,
    );

    headers.insert(
        HeaderName::from_str("Digest").map_err(FlymodelError::internal_error)?,
        HeaderValue::from_str(&format!("sha256={hash}")).map_err(FlymodelError::internal_error)?,
    );

    Ok(resp)
}

struct CommonExperimentCte {
    experiment: entities::experiment::Model,
    model_version: entities::model_version::Model,
    bucket: entities::bucket::Model,
}

async fn get_common_from_experiment(
    experiment_id: i64,
    experiment: Data<DataLoader<DbLoader<entities::experiment::Model>>>,
    namespaces: Data<DataLoader<DbLoader<entities::namespace::Model>>>,
    versions: Data<DataLoader<DbLoader<entities::model_version::Model>>>,
    buckets: Data<DataLoader<DbLoader<entities::bucket::Model>>>,
) -> Result<CommonExperimentCte, FlymodelError> {
    let on_missing = || FlymodelError::InvalidResourceId(experiment_id);

    let (model_version, experiment) = experiment
        .loader()
        .single_model_version(experiment_id)
        .await?
        .ok_or_else(on_missing)?;

    let state = versions.loader().state(&model_version).await?.expect("ok");

    let namespace =
        DbLoader::<entities::namespace::Model>::namespace_of_model(&namespaces, &model_version)
            .await?
            .ok_or_else(on_missing)?;

    let bucket = entities::bucket::Entity::find()
        .filter(entities::bucket::Column::Namespace.eq(namespace.id))
        .filter(
            Expr::expr(Expr::col(entities::bucket::Column::Role).cast_as(Alias::new("varchar")))
                .eq(state.state.into_value().as_str().to_string()),
        )
        .one(&buckets.loader().db)
        .await
        .map_err(|err| FlymodelError::DbLoaderError(Arc::new(err)))?
        .ok_or_else(on_missing)?;

    Ok(CommonExperimentCte {
        experiment,
        model_version,
        bucket,
        // namespace,
        // state,
    })
}

#[routes]
#[post("/upload/experiment-artifact")]
pub async fn upload_experiment_artifact(
    storage: Data<Arc<StorageOrchestrator>>,
    namespaces: Data<DataLoader<DbLoader<entities::namespace::Model>>>,
    experiment: Data<DataLoader<DbLoader<entities::experiment::Model>>>,
    artifact: Data<DataLoader<DbLoader<entities::experiment_artifact::Model>>>,
    buckets: Data<DataLoader<DbLoader<entities::bucket::Model>>>,
    versions: Data<DataLoader<DbLoader<entities::model_version::Model>>>,
    blobs: Data<DataLoader<DbLoader<entities::object_blob::Model>>>,
    MultipartForm(form): MultipartForm<UploadExperimentArtifact>,
) -> actix_web::Result<impl Responder> {
    let data = form.artifact;
    let bs = read_bts(form.file)?;
    let sz = bs.len() as i64;
    if sz == 0 {
        return Err(
            FlymodelError::NonDeterministicError("Uploads must contain data".into()).into(),
        );
    }

    let cte =
        get_common_from_experiment(data.experiment, experiment, namespaces, versions, buckets)
            .await?;

    let hash = sha256::digest(&*bs);

    let key = format!(
        "experiments/{id}/{artifact}",
        id = cte.experiment.id,
        artifact = data.blob.artifact_name
    );

    debug!("upload size: {sz}");

    let sink = storage
        .get(&cte.bucket.name)
        .ok_or(FlymodelError::RuntimeDependencyError(format!(
            "missing {} bucket configurations",
            cte.bucket.name
        )))?;

    let upload_res = sink.put(key.clone(), bs).await?;

    let blob = blobs
        .as_ref()
        .loader()
        .create_new_blob(
            cte.bucket.id,
            key,
            upload_res.expect("version id"),
            &data.blob,
            sz,
            hash,
        )
        .await?;

    let created = artifact
        .as_ref()
        .loader()
        .create_new_artifact(&cte.experiment, &cte.model_version, &blob, &data.blob)
        .await?;

    debug!("created: {:#?}", created);

    Ok(web::Json(created))
}
