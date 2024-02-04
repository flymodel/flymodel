use std::sync::Arc;

use crate::{
    artifacts::{UploadExperiment, UploadExperimentArgs},
    experiment::experiment::ExperimentError,
    maybe,
};

use crate::experiment::state::*;

use flymodel_graphql::gql::create_experiment;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use rust_fsm::*;
use tokio::sync::Mutex;
use tracing::debug;

#[pyclass]
#[derive(Clone)]
pub struct Experiment {
    experiment: Arc<Mutex<Option<flymodel_graphql::gql::create_experiment::Experiment>>>,
    client: Arc<crate::py::PythonClient>,
    state: Arc<Mutex<StateMachine<ExperimentState>>>,
    args: Arc<create_experiment::CreateExperimentVariables>,
}

impl IntoPy<PyErr> for ExperimentError {
    fn into_py(self, _py: Python) -> PyErr {
        PyRuntimeError::new_err(self.to_string())
    }
}

impl From<ExperimentError> for PyErr {
    fn from(value: ExperimentError) -> Self {
        PyRuntimeError::new_err(value.to_string())
    }
}

impl Experiment {
    async fn consume(&self, state: ExperimentStateInput) -> Result<(), ExperimentError> {
        self.state.clone().lock_owned().await.consume(&state)?;
        Ok(())
    }
}

#[pymethods]
impl Experiment {
    #[new]
    pub fn new(
        client: crate::py::PythonClient,
        args: create_experiment::CreateExperimentVariables,
    ) -> Self {
        let state = Arc::new(Mutex::new(StateMachine::new()));
        Self {
            state,
            client: Arc::new(client),
            args: Arc::new(args),
            experiment: Arc::new(Mutex::new(None)),
        }
    }

    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
        let re = slf.clone();
        slf.client.runtime().pyfut(py, async move {
            re.consume(ExperimentStateInput::Started)
                .await
                .map_err(PyErr::from)?;
            // let experiment = client.create_experiment(args).await?;
            Ok(())
        })
    }

    fn __iter__<'py>(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __await__<'py>(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }
    fn save_artifact<'py>(
        slf: PyRef<'py, Self>,
        py: Python<'py>,
        artifact: UploadExperimentArgs,
        data: Vec<u8>,
    ) -> PyResult<&'py PyAny> {
        let command = UploadExperiment::new(artifact, data);
        let re = slf.clone();
        slf.client.runtime().pyfut(py, async move {
            let resp = re
                .client
                .shared
                .upload("/upload/experiment-artifact", command)
                .await?;
            #[cfg(debug_assertions)]
            debug!("{:#?}", resp);
            Python::with_gil(|py| match resp {
                maybe::Result::Ok(resp) => Ok(resp),
                maybe::Result::Err(e) => Err(e.into_py(py)),
            })
        })
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        exc_type: Option<PyObject>,
        exc_value: Option<PyObject>,
        traceback: Option<PyObject>,
    ) -> PyResult<&'py PyAny> {
        tracing::info!("{:#?}\n{:#?}\n{:#?}", exc_type, exc_value, traceback);
        let this = self.clone();
        self.client.runtime().pyfut(py, async move {
            this.consume(ExperimentStateInput::WaitClose)
                .await
                .map_err(PyErr::from)?;
            Ok(())
        })
    }
}
