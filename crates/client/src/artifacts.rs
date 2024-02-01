use std::marker::PhantomData;

use flymodel_graphql::enums::*;
use flymodel_macros::hybrid_feature_class;
use reqwest::multipart::{Form, Part};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[hybrid_feature_class(wasm = true, python = true)]
#[derive(Serialize, Clone, Debug)]
pub struct UploadRequestParams {
    pub artifact_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ArchiveFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encode: Option<ArchiveCompression>,
}

#[cfg(feature = "python")]
#[pyo3::prelude::pymethods]
impl UploadRequestParams {
    #[new]
    #[pyo3(signature = (artifact_name, format = None, encode = None))]
    pub fn new(
        artifact_name: String,
        format: Option<ArchiveFormat>,
        encode: Option<ArchiveCompression>,
    ) -> Self {
        Self {
            artifact_name,
            format,
            encode,
        }
    }
}

trait Command<D: Serialize, R: DeserializeOwned> {
    const NAME: &'static str;
}

pub struct CommandDescriptor<D: Serialize, R: DeserializeOwned> {
    pub(crate) artifact: D,
    data: Vec<u8>,
    response: PhantomData<R>,
}

impl<D: Serialize, R: DeserializeOwned> TryInto<Form> for CommandDescriptor<D, R> {
    type Error = crate::Error;

    fn try_into(self) -> Result<Form, Self::Error> {
        let mut form = Form::new();
        form = form.part(
            "file",
            Part::bytes(self.data)
                .mime_str("application/octet-stream")
                .map_err(Self::Error::UploadError)?,
        );
        form = form.part(
            "artifact",
            Part::bytes(serde_json::to_vec(&self.artifact)?)
                .mime_str("application/json")
                .map_err(Self::Error::UploadError)?,
        );
        Ok(form)
    }
}

impl<D: Serialize, R: DeserializeOwned> CommandDescriptor<D, R> {
    pub fn new(artifact: D, data: Vec<u8>) -> Self {
        Self {
            data,
            artifact,
            response: PhantomData,
        }
    }
}

macro_rules! upload_impl {
    ($name: ident, [$((
        $(#[$($m: tt)*])*
        $arg: ident: $typ: ty)), + $(,)?]) => {
        paste::paste! {
            #[hybrid_feature_class(wasm = true, python = true)]
            #[derive(Serialize, Clone, Debug)]
            pub struct [<Upload $name Args>] {
                $(
                    $(#[$($m)*])*
                    pub $arg: $typ,
                )*
                #[serde(flatten)]
                pub blob: UploadRequestParams,
            }


            #[cfg(feature = "python")]
            #[pyo3::prelude::pymethods]
            impl [<Upload $name Args>] {
                #[new]

                fn new(
                    params: UploadRequestParams,
                    $(
                        $arg: $typ,
                    )*
                ) -> Self {
                    Self {
                        blob: params,
                        $(
                            $arg: $arg.into(),
                        )*
                    }
                }
             }

            pub type [<Upload $name>] = CommandDescriptor<[<Upload $name Args>], [<$name Response>]>;
        }
    };
}

#[hybrid_feature_class(wasm = true, python = true)]
#[derive(Deserialize, Debug)]
pub struct ExperimentResponse {
    pub blob: i64,
    pub experiment_id: i64,
    pub id: i64,
    pub name: String,
    pub version_id: i64,
}

#[derive(Deserialize)]
pub struct ModelVersionResponse {
    #[serde(flatten)]
    pub value: serde_json::Value,
}

upload_impl!(Experiment, [
    (experiment: i64),
]);

upload_impl!(ModelVersion, [
    (model_version: i64), (
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Vec<u8>>
)]);

#[cfg(test)]
mod test {
    use super::UploadModelVersion;

    #[test]
    fn upload_model_version_ser() -> anyhow::Result<()> {
        let up = UploadModelVersion::new(
            super::UploadModelVersionArgs {
                model_version: 1,
                extra: None,
                blob: super::UploadRequestParams {
                    artifact_name: "Some Name".into(),
                    format: None,
                    encode: None,
                },
            },
            vec![],
        );

        let ser = serde_json::to_string(&up.artifact)?;
        assert_eq!(ser, r#"{"model_version":1,"artifact_name":"Some Name"}"#);
        Ok(())
    }

    #[test]
    fn upload_experiment_ser() -> anyhow::Result<()> {
        let up = super::UploadExperiment::new(
            super::UploadExperimentArgs {
                experiment: 1,
                blob: super::UploadRequestParams {
                    artifact_name: "Some Name".into(),
                    format: None,
                    encode: None,
                },
            },
            vec![],
        );

        let ser = serde_json::to_string(&up.artifact)?;
        assert_eq!(ser, r#"{"experiment":1,"artifact_name":"Some Name"}"#);
        Ok(())
    }
}
