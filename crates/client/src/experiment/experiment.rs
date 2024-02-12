use super::state::*;
use crate::artifacts::{self, PartialUploadExperimentArgs};
use flymodel_graphql::gql::create_experiment;
use rust_fsm::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type ExperimentFunction =
    | ((experiment: Experiment) => Promise<void>)
    | ((experiment: Experiment) => void);
"#;

#[derive(Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Experiment {
    experiment: Arc<flymodel_graphql::gql::create_experiment::Experiment>,
    client: Arc<crate::client::Client>,
    state: Arc<Mutex<StateMachine<ExperimentState>>>,
}

#[derive(thiserror::Error, Debug)]
pub enum ExperimentError {
    #[error("{0}")]
    ClientError(#[from] crate::client::Error),

    #[error("Invalid state: {0}")]
    InvalidStateError(String),

    #[error("Transition error: {0}")]
    TransitionError(#[from] TransitionImpossibleError),

    #[error("Read state error: {0}")]
    StateReadError(#[from] tokio::sync::TryLockError),

    #[cfg(feature = "wasm")]
    #[error("Js runtime error: {0}")]
    JsRuntimeError(String),

    #[cfg(feature = "python")]
    #[error("Invalid state call: {0}")]
    InvalidStateCaller(&'static str),
}

#[cfg(feature = "wasm")]
impl Into<wasm_bindgen::JsValue> for ExperimentError {
    fn into(self) -> wasm_bindgen::JsValue {
        wasm_bindgen::JsValue::from_str(&self.to_string())
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl Experiment {
    #[cfg(not(feature = "wasm"))]
    pub async fn new(
        client: Arc<crate::client::Client>,
        args: create_experiment::CreateExperimentVariables,
    ) -> Result<Self, ExperimentError> {
        let experiment = client.create_experiment(args).await?;
        let state = Arc::new(Mutex::new(StateMachine::new()));
        Ok(Self {
            client,
            state,
            experiment: Arc::new(experiment.create_experiment),
        })
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(constructor)]
    pub async fn new(
        client: crate::client::Client,
        args: create_experiment::CreateExperimentVariables,
    ) -> Result<Experiment, ExperimentError> {
        let client = Arc::new(client);
        let state = Arc::new(Mutex::new(StateMachine::new()));
        let experiment = client.create_experiment(args).await?;
        Ok(Self {
            client,
            state,
            experiment: Arc::new(experiment.create_experiment),
        })
    }

    #[allow(dead_code)]
    async fn consume(&self, state: ExperimentStateInput) -> Result<(), ExperimentError> {
        consume_mu(self.state.clone(), state).await
    }

    #[cfg(feature = "wasm")]
    async fn run_unguarded(self, experiment_fn: &js_sys::Function) -> Result<(), ExperimentError> {
        self.consume(ExperimentStateInput::Started).await?;
        self.consume(ExperimentStateInput::Entered).await?;
        let value = wasm_bindgen::JsValue::null();
        tracing::debug!("function: {:#?}", experiment_fn);
        let fut = experiment_fn
            .call1(&value, &JsValue::from(self.clone()))
            .map_err(|err| ExperimentError::JsRuntimeError(format!("{:#?}", err)))?;
        if js_sys::Promise::instanceof(&fut) {
            tracing::debug!("future: {:#?}", fut);
            wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(fut))
                .await
                .map_err(|err| ExperimentError::JsRuntimeError(format!("{:#?}", err)))?;
        } else {
            tracing::debug!("complete");
        }
        self.consume(ExperimentStateInput::WaitClose).await?;
        Ok(())
    }

    #[cfg(feature = "wasm")]
    pub async fn run(self, experiment_fn: &js_sys::Function) -> Result<(), ExperimentError> {
        Ok(self.run_unguarded(experiment_fn).await?)
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(js_name = "saveArtifact"))]
    pub async fn save_artifact(
        &self,
        artifact: PartialUploadExperimentArgs,
        data: Vec<u8>,
    ) -> Result<artifacts::ExperimentResponse, ExperimentError> {
        if !matches!(
            self.state.clone().try_lock()?.state(),
            ExperimentStateState::Tests
        ) {
            return Err(ExperimentError::InvalidStateError(format!(
                "Cannot save an artifact in non-started state",
            )));
        }
        Ok(self
            .client
            .upload_experiment_artifact(artifact.with_context(self.experiment.id.into()), data)
            .await
            .map_err(ExperimentError::from)?)
    }
}
