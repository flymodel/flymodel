use std::sync::Arc;

use crate::{
    artifacts::{download_with_blob, guarded_upload, read_bts, DownloadParams},
    params_for,
};
use actix_web::{
    routes,
    web::{Data, Json, Query},
    Responder,
};

use async_graphql::dataloader::{DataLoader, Loader};
use flymodel::errs::FlymodelError;
use flymodel_entities::{
    db::DbLoader,
    entities::{self},
};

use flymodel_registry::storage::StorageOrchestrator;
use sea_orm::{DbErr, EntityTrait};
use serde::Deserialize;

use actix_multipart::form::{self, tempfile::TempFile, MultipartForm};
use tracing::debug;

params_for!(ModelVersion, [(model_version: i64), (extra: Option<serde_json::Value>)]);

#[derive(Clone, Debug)]
struct CommonModelCte {
    model_version: entities::model_version::Model,
    bucket: entities::bucket::Model,
}

async fn get_common_from_model_version<
    FM: Fn() -> FlymodelError + Copy,
    FE: Fn(DbErr) -> FlymodelError + Copy,
>(
    model_version_id: i64,
    namespaces: Data<DataLoader<DbLoader<entities::namespace::Model>>>,
    versions: Data<DataLoader<DbLoader<entities::model_version::Model>>>,
    buckets: Data<DataLoader<DbLoader<entities::bucket::Model>>>,
    on_missing: FM,
    on_error: FE,
) -> Result<CommonModelCte, FlymodelError> {
    let model_version = versions
        .load_one(model_version_id)
        .await
        .map_err(FlymodelError::DbLoaderError)?
        .ok_or_else(on_missing)?;

    let state = versions.loader().state(&model_version).await?.expect("ok");

    let (_model, namespace) = entities::model::Entity::find_by_id(model_version.model_id)
        .find_also_related(entities::namespace::Entity)
        .one(&namespaces.loader().db)
        .await
        .map_err(on_error)?
        .ok_or_else(on_missing)?;

    let namespace = namespace.ok_or_else(on_missing)?;

    let bucket = buckets
        .loader()
        .find_by_model(&namespace, &state, on_missing)
        .await?;

    Ok(CommonModelCte {
        // model,
        model_version,
        bucket,
    })
}

#[routes]
#[post("/upload/model-version-artifact")]
pub async fn upload_model_version_artifact(
    MultipartForm(form): MultipartForm<UploadModelVersionArtifact>,

    storage: Data<Arc<StorageOrchestrator>>,
    namespaces: Data<DataLoader<DbLoader<entities::namespace::Model>>>,
    versions: Data<DataLoader<DbLoader<entities::model_version::Model>>>,
    buckets: Data<DataLoader<DbLoader<entities::bucket::Model>>>,
    blobs: Data<DataLoader<DbLoader<entities::object_blob::Model>>>,
) -> actix_web::Result<impl Responder> {
    let data = form.artifact;
    let on_err = |err| FlymodelError::DbLoaderError(Arc::new(err));
    let on_missing = || FlymodelError::InvalidResourceId(data.model_version);
    let cte = get_common_from_model_version(
        data.model_version,
        namespaces,
        versions,
        buckets,
        on_missing,
        on_err,
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
    let hash = sha256::digest(&*bs);

    let key = format!(
        "model_versions/{id}/{artifact}",
        id = cte.model_version.id,
        artifact = data.blob.artifact_name
    );

    let created = guarded_upload(
        sink,
        bs,
        &blobs.loader().db,
        key.clone(),
        |tx, version_id| {
            Box::pin(async move {
                let tx = tx;
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
                Ok(
                    DbLoader::<entities::model_artifact::Model>::create_new_artifact(
                        tx,
                        &cte.model_version,
                        &blob,
                        &data.blob,
                        data.extra.clone(),
                    )
                    .await?,
                )
            })
        },
    )
    .await?;

    Ok(Json(created))
}

#[routes]
#[get("/download/model-version-artifact")]
pub async fn download_model_version_artifact(
    storage: Data<Arc<StorageOrchestrator>>,
    params: Query<DownloadParams>,

    namespaces: Data<DataLoader<DbLoader<entities::namespace::Model>>>,
    versions: Data<DataLoader<DbLoader<entities::model_version::Model>>>,
    artifact: Data<DataLoader<DbLoader<entities::model_artifact::Model>>>,
    buckets: Data<DataLoader<DbLoader<entities::bucket::Model>>>,
    blobs: Data<DataLoader<DbLoader<entities::object_blob::Model>>>,
) -> actix_web::Result<impl Responder> {
    let on_err = |err| FlymodelError::DbLoaderError(Arc::new(err));
    let on_missing = || FlymodelError::InvalidResourceId(params.artifact_id);

    let artifact = artifact
        .loader()
        .load(&[params.artifact_id])
        .await
        .map_err(|err| FlymodelError::DbLoaderError(err))?;

    let artifact = artifact.get(&params.artifact_id).ok_or_else(on_missing)?;

    let cte = get_common_from_model_version(
        artifact.version_id,
        namespaces,
        versions,
        buckets,
        on_missing,
        on_err,
    )
    .await?;

    let blobref = blobs
        .loader()
        .load(&[artifact.blob])
        .await
        .map_err(|err| FlymodelError::DbLoaderError(err))?;

    let blobref = blobref.get(&artifact.blob).ok_or_else(on_missing)?;

    Ok(download_with_blob(
        blobref,
        &cte.bucket,
        storage.as_ref(),
        artifact.name.clone(),
    )
    .await?)
}
