#![allow(non_snake_case)]
use std::fmt::Debug;

use reqwest::Url;

use cfg_if::cfg_if;
use cynic::{GraphQlError, GraphQlResponse, MutationBuilder, Operation, QueryBuilder};
use flymodel_graphql::gql::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use flymodel_dev::config_attr;
use flymodel_macros::hybrid_feature_class;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(all(not(feature = "wasm"), feature = "python"))]
use pyo3::prelude::*;

#[hybrid_feature_class(wasm = true, py_getters = false)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
}

#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(into_wasm_abi))]
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorExt {
    #[serde(flatten)]
    pub rest: serde_json::Value,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("GraphQL error: {0:#?}")]
    ExtErrorsFound(Vec<GraphQlError<ErrorExt>>),

    #[error("Received empty result when expecting data")]
    EmptyResult,

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Base url error: {0}")]
    BaseUrlError(#[from] url::ParseError),

    #[cfg(feature = "python")]
    #[error("Python implementation error: {0}")]
    PyErr(#[from] pyo3::PyErr),
}

cfg_if! {
    if #[cfg(feature = "wasm")] {
        impl Into<wasm_bindgen::JsValue> for Error {
            fn into(self) -> JsValue {
                JsValue::from_str(&self.to_string())
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "python")] {
        impl From<Error> for PyErr {
            fn from(value: Error) -> PyErr {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(value.to_string())
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg_attr(feature = "tracing", tracing::instrument(level = "debug"))]
fn raise_for<T: Debug>(result: GraphQlResponse<T, ErrorExt>) -> Result<T> {
    if let Some(errors) = result.errors {
        return Err(Error::ExtErrorsFound(errors));
    } else if let Some(data) = result.data {
        return Ok(data);
    } else {
        Err(Error::EmptyResult)
    }
}

impl Client {
    pub async fn post<T: Serialize, R: DeserializeOwned>(&self, url: &str, data: T) -> Result<R> {
        let url = self.base_url.join(url)?;

        #[cfg(all(feature = "tracing", debug_assertions))]
        tracing::trace! {
            name: "request",
            "{url} [{method}]: {data}",
            url = url.as_str(),
            method = "POST",
            data = serde_json::to_string(&data).unwrap()
        };
        #[cfg(all(feature = "tracing", not(debug_assertions)))]
        tracing::trace! {
            name: "request",
            "{url} [{method}]",
            url = url.as_str(),
            method = "POST"
        };

        Ok(self
            .client
            .post(url)
            .json(&data)
            .send()
            .await?
            .json()
            .await?)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "info", skip(self)))]
    #[inline]
    pub async fn perform_mutation<Vars, M: MutationBuilder<Vars> + DeserializeOwned>(
        &self,
        model: Vars,
    ) -> Result<M>
    where
        Vars: Debug,
        M: Serialize + Debug,
        Operation<M, Vars>: Serialize,
    {
        let op = M::build(model);
        Ok(raise_for(self.post("/graphql", &op).await?)?)
    }

    #[inline]
    pub async fn perform_query<Vars, Q: QueryBuilder<Vars> + DeserializeOwned>(
        &self,
        model: Vars,
    ) -> Result<Q>
    where
        Q: Serialize + Debug,
        Operation<Q, Vars>: Serialize,
    {
        let op = Q::build(model);
        Ok(raise_for(self.post("/graphql", &op).await?)?)
    }

    #[cfg(not(feature = "wasm"))]
    pub fn new(base_url: &str) -> Result<Client> {
        Client::new_common(base_url)
    }

    #[inline]
    pub(crate) fn new_common(base_url: &str) -> Result<Self> {
        Ok(Self {
            base_url: base_url.parse()?,
            client: reqwest::ClientBuilder::new().build().expect("ok"),
        })
    }
}

config_attr! {
    if #[cfg(feature = "wasm")] {
        #[wasm_bindgen]
    } for {
        impl Client {
            #[cfg(feature = "wasm")]
            #[wasm_bindgen(constructor)]
            pub fn new(base_url: &str) -> Result<Client> {
                Client::new_common(base_url)
            }

            pub async fn create_bucket(&self, bucket: create_bucket::CreateBucketVariables) -> Result<create_bucket::CreateBucket> {
                self.perform_mutation(bucket).await
            }

            pub async fn delete_bucket(&self, bucket: delete_bucket::DeleteBucketVariables) -> Result<delete_bucket::DeleteBucket> {
                self.perform_mutation(bucket).await
            }

            pub async fn create_namespace(&self, namespace: create_namespace::CreateNamespaceVariables) -> Result<create_namespace::CreateNamespace> {
                self.perform_mutation(namespace).await
            }

            pub async fn delete_namespace(&self, namespace: delete_namespace::DeleteNamespaceVariables) -> Result<delete_namespace::DeleteNamespace> {
                self.perform_mutation(namespace).await
            }

            pub async fn update_namespace(&self, namespace: update_namespace::UpdateNamespaceVariables) -> Result<update_namespace::UpdateNamespace> {
                self.perform_mutation(namespace).await
            }

            pub async fn create_model(
                &self,
                model: create_model::CreateModelVariables,
            ) -> Result<create_model::CreateModel> {
                self.perform_mutation(model).await
            }

            pub async fn delete_model(&self, model: delete_model::DeleteModelVariables) -> Result<delete_model::DeleteModel> {
                self.perform_mutation(model).await
            }

            pub async fn update_model(&self, model: update_model::UpdateModelVariables) -> Result<update_model::UpdateModel> {
                self.perform_mutation(model).await
            }

            pub async fn create_model_version(
                &self,
                version: create_model_version::CreateModelVersionVariables,
            ) -> Result<create_model_version::CreateModelVersion> {
                self.perform_mutation(version).await
            }

            pub async fn delete_model_version(
                &self,
                version: delete_model_version::DeleteModelVersionVariables,
            ) -> Result<delete_model_version::DeleteModelVersion> {
                self.perform_mutation(version).await
            }

            pub async fn update_model_version_state(&self, state: update_model_version_state::UpdateModelVersionStateVariables) -> Result<update_model_version_state::UpdateModelVersionState> {
                self.perform_mutation(state).await
            }

            pub async fn create_experiment(
                &self,
                experiment: create_experiment::CreateExperimentVariables,
            ) -> Result<create_experiment::CreateExperiment> {
                self.perform_mutation(experiment).await
            }

            pub async fn delete_experiment(
                &self,
                experiment: delete_experiment::DeleteExperimentVariables,
            ) -> Result<delete_experiment::DeleteExperiment> {
                self.perform_mutation(experiment).await
            }

            pub async fn query_namespaces(&self, vars: query_namespaces::QueryNamespacesVariables) -> Result<query_namespaces::QueryNamespaces> {
                self.perform_query(vars).await
            }

            pub async fn query_buckets(&self, vars: query_buckets::QueryBucketsVariables) -> Result<query_buckets::QueryBuckets> {
                self.perform_query(vars).await
            }

            pub async fn query_namespace_models(
                &self,
                vars: query_models::NamespaceModelsVariables,
            ) -> Result<query_models::NamespaceModels> {
                self.perform_query(vars).await
            }

            pub async fn query_experiment(&self, vars: query_experiment::QueryExperimentVariables) -> Result<query_experiment::QueryExperiment> {
                self.perform_query(vars).await
            }

        }
    }

}
