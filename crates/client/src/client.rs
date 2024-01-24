#![allow(non_snake_case)]
use reqwest::Url;

use crate::wasm::*;
use cfg_if::cfg_if;
use cynic::{GraphQlError, GraphQlResponse, MutationBuilder, Operation, QueryBuilder};
use flymodel_graphql::gql::{
    create_experiment,
    create_model::{self},
    create_model_version, query_buckets, query_models, query_namespaces,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use flymodel_dev::config_attr;

config_attr! {
    if #[cfg(feature = "wasm")] {
        #[wasm_bindgen]
    } for {
        pub struct FlymodelClient {
            base_url: Url,
            client: reqwest::Client,
        }
    }
}

config_attr! {
    if #[cfg(feature = "wasm")] {
        #[derive(tsify::Tsify)]
        #[tsify(into_wasm_abi)]
    } for {
        #[derive(Debug, Deserialize, Serialize)]
        pub struct ErrorExt {
            #[serde(flatten)]
            pub rest: serde_json::Value,
        }
    }
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

type Result<T> = std::result::Result<T, Error>;

fn raise_for<T>(result: GraphQlResponse<T, ErrorExt>) -> Result<T> {
    if let Some(errors) = result.errors {
        return Err(Error::ExtErrorsFound(errors));
    } else if let Some(data) = result.data {
        return Ok(data);
    } else {
        Err(Error::EmptyResult)
    }
}

impl FlymodelClient {
    pub async fn post<T: Serialize, R: DeserializeOwned>(&self, url: &str, data: T) -> Result<R> {
        let url = self.base_url.join(url)?;
        #[cfg(debug_assertions)]
        {
            log(format!(
                "[POST]: url={}, data={:#?}",
                url.as_str(),
                serde_json::to_string(&data)
                    .expect("operation to serialize")
                    .replace("\n", " ")
            )
            .as_str());
        }
        Ok(self
            .client
            .post(url)
            .json(&data)
            .send()
            .await?
            .json()
            .await?)
    }

    #[inline]
    pub async fn perform_mutation<Vars, M: MutationBuilder<Vars> + DeserializeOwned>(
        &self,
        model: Vars,
    ) -> Result<M>
    where
        M: Serialize,
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
        Q: Serialize,
        Operation<Q, Vars>: Serialize,
    {
        let op = Q::build(model);
        Ok(raise_for(self.post("/graphql", &op).await?)?)
    }

    #[inline]
    fn new_common(base_url: &str) -> Result<Self> {
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
        impl FlymodelClient {
            #[cfg(feature = "wasm")]
            #[wasm_bindgen(constructor)]
            pub fn new(base_url: &str) -> Result<FlymodelClient> {
                FlymodelClient::new_common(base_url)
            }

            #[cfg(not(feature = "wasm"))]
            pub fn new(base_url: &str) -> Result<FlymodelClient> {
                FlymodelClient::new_common(base_url)
            }

            pub async fn create_model(
                &self,
                model: create_model::CreateModelVariables,
            ) -> Result<create_model::CreateModel> {
                self.perform_mutation(model).await
            }

            pub async fn create_model_version(
                &self,
                model: create_model_version::CreateModelVersionVariables,
            ) -> Result<create_model_version::CreateModelVersion> {
                self.perform_mutation(model).await
            }

            pub async fn create_experiment(
                &self,
                model: create_experiment::CreateExperimentVariables,
            ) -> Result<create_experiment::CreateExperiment> {
                self.perform_mutation(model).await
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
        }
    }

}
