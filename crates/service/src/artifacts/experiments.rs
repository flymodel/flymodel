use crate::{
    artifacts::{download_with_blob, guarded_upload, read_bts, DownloadParams},
    params_for,
};
use actix_web::{
    routes,
    web::{self, Data, Query},
    Responder,
};

use async_graphql::dataloader::{DataLoader, Loader};

use flymodel::errs::FlymodelError;
use flymodel_entities::{
    db::DbLoader,
    entities::{self},
};
use flymodel_registry::storage::StorageOrchestrator;
use sea_orm::ActiveEnum;
use serde::Deserialize;
use std::sync::Arc;
use tracing::debug;

use actix_multipart::form::{self, tempfile::TempFile, MultipartForm};

params_for!(Experiment, [(experiment: i64)]);

struct CommonExperimentCte {
    experiment: entities::experiment::Model,
    model_version: entities::model_version::Model,
    bucket: entities::bucket::Model,
}

async fn get_common_from_experiment<FM: Fn() -> FlymodelError + Copy>(
    experiment_id: i64,
    experiment: Data<DataLoader<DbLoader<entities::experiment::Model>>>,
    namespaces: Data<DataLoader<DbLoader<entities::namespace::Model>>>,
    versions: Data<DataLoader<DbLoader<entities::model_version::Model>>>,
    buckets: Data<DataLoader<DbLoader<entities::bucket::Model>>>,
    on_missing: FM,
) -> Result<CommonExperimentCte, FlymodelError> {
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

    let bucket = buckets
        .loader()
        .find_by_model(&namespace, &state, on_missing)
        .await?;

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
    MultipartForm(form): MultipartForm<UploadExperimentArtifact>,

    storage: Data<Arc<StorageOrchestrator>>,
    namespaces: Data<DataLoader<DbLoader<entities::namespace::Model>>>,
    experiment: Data<DataLoader<DbLoader<entities::experiment::Model>>>,
    _artifact: Data<DataLoader<DbLoader<entities::experiment_artifact::Model>>>,
    buckets: Data<DataLoader<DbLoader<entities::bucket::Model>>>,
    versions: Data<DataLoader<DbLoader<entities::model_version::Model>>>,
    blobs: Data<DataLoader<DbLoader<entities::object_blob::Model>>>,
) -> actix_web::Result<impl Responder> {
    let data = form.artifact;
    let on_missing = || FlymodelError::InvalidResourceId(data.experiment);
    let cte = get_common_from_experiment(
        data.experiment,
        experiment,
        namespaces,
        versions,
        buckets,
        on_missing,
    )
    .await?;

    let sink = storage
        .get(&cte.bucket.name)
        .ok_or(FlymodelError::RuntimeDependencyError(format!(
            "missing {} bucket configurations",
            cte.bucket.name
        )))?;

    let bs = read_bts(form.file)?;
    let sz = bs.len();
    debug!("upload size: {}", sz);

    let hash = sha256::digest(&*bs);

    let key = format!(
        "experiments/{id}/{artifact}",
        id = cte.experiment.id,
        artifact = data.blob.artifact_name
    );

    let created = guarded_upload(
        sink,
        bs,
        &blobs.loader().db,
        key.clone(),
        |tx, version_id| {
            Box::pin(async move {
                let blob = DbLoader::<entities::object_blob::Model>::create_new_blob(
                    tx,
                    cte.bucket.id,
                    key,
                    version_id.expect("version id"),
                    &data.blob,
                    sz as i64,
                    hash,
                )
                .await?;

                let created =
                    DbLoader::<entities::experiment_artifact::Model>::create_new_artifact(
                        tx,
                        &cte.experiment,
                        &cte.model_version,
                        &blob,
                        &data.blob,
                    )
                    .await?;
                Ok(created)
            })
        },
    )
    .await?;

    Ok(web::Json(created))
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
        on_missing,
    )
    .await?;

    let blobref = blobs
        .loader()
        .load(&[artifact.blob])
        .await
        .map_err(on_err)?;

    let blobref = blobref.get(&artifact.blob).ok_or_else(on_missing)?;

    Ok(download_with_blob(
        blobref,
        &cte.bucket,
        storage.as_ref(),
        artifact.name.clone(),
    )
    .await?)
}
