use std::sync::Arc;

use crate::{
    artifacts::{UploadExperiment, UploadExperimentArgs},
    maybe,
};

use super::fsm::ExperimentStateInput;

use pyo3::{exceptions::PyRuntimeError, prelude::*};
use rust_fsm::*;
use tokio::sync::Mutex;

use super::fsm::*;

#[pyclass]
#[derive(Clone)]
pub struct Experiment {
    client: Arc<super::PythonClient>,
    state: Arc<Mutex<StateMachine<ExperimentState>>>,
}

#[derive(thiserror::Error, Debug)]
enum ExperimentError {
    #[error("Transition error: {0}")]
    TransitionError(#[from] TransitionImpossibleError),
}

impl IntoPy<PyErr> for ExperimentError {
    fn into_py(self, _py: Python) -> PyErr {
        PyRuntimeError::new_err(self.to_string())
    }
}

#[pymethods]
impl Experiment {
    #[new]
    fn new(client: PyRef<super::PythonClient>) -> Self {
        let this = Self {
            client: Arc::new(client.clone()),
            state: Arc::new(Mutex::new(StateMachine::new())),
        };
        this
    }

    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
        let exp = slf.clone();
        slf.client.runtime().pyfut(py, async move {
            let mut guard = exp.state.clone().lock_owned().await;
            Python::with_gil(|py| {
                guard
                    .consume(&ExperimentStateInput::Started)
                    .map_err(|err| ExperimentError::from(err).into_py(py))
            })?;
            Ok(exp)
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
        let client = slf.client.shared.clone();
        slf.client.runtime().pyfut(py, async move {
            let resp = client
                .upload("/upload/experiment-artifact", command)
                .await?;
            println!("{:#?}", resp);
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
        let _this = self.clone();
        self.client.runtime().pyfut(py, async move { Ok(()) })
    }
}
