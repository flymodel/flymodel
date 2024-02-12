use std::sync::Arc;

use crate::{artifacts::PartialUploadExperimentArgs, experiment::experiment::ExperimentError};

use crate::experiment::state::*;

use flymodel_graphql::gql::{self, create_experiment};
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
        consume_mu(self.state.clone(), state).await
    }

    fn ensure_aenter() -> PyErr {
        Python::with_gil(|py| {
            ExperimentError::InvalidStateCaller(
                r#""__aenter__(..)" expected. ensure you use "Experiment" within a with block."#,
            )
            .into_py(py)
        })
    }

    async fn py_experiment(&self) -> PyResult<gql::create_experiment::Experiment> {
        self.experiment
            .lock()
            .await
            .clone()
            .ok_or_else(Self::ensure_aenter)
    }

    fn blocking_experiment(&self) -> PyResult<gql::create_experiment::Experiment> {
        self.experiment
            .blocking_lock()
            .clone()
            .ok_or_else(Self::ensure_aenter)
    }

    async fn fail_on<T, E: Into<PyErr>>(&self, res: Result<T, E>) -> PyResult<T> {
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                self.consume(ExperimentStateInput::Failed)
                    .await
                    .map_err(|err| Python::with_gil(|py| err.into_py(py)))?;
                Err(e.into())
            }
        }
    }
}

#[pymethods]
impl Experiment {
    #[new]
    pub fn new(
        client: crate::py::PythonClient,
        args: create_experiment::CreateExperimentVariables,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(StateMachine::new())),
            client: Arc::new(client),
            args: Arc::new(args),
            experiment: Arc::new(Mutex::new(None)),
        }
    }

    fn __aenter__<'py>(slf: PyRef<'py, Self>, py: Python<'py>) -> PyResult<&'py PyAny> {
        let this = slf.clone();
        slf.client.runtime().pyfut(py, async move {
            this.consume(ExperimentStateInput::Started)
                .await
                .map_err(PyErr::from)?;

            let exp = this
                .fail_on(
                    this.client
                        .shared
                        .create_experiment(this.args.as_ref().clone())
                        .await,
                )
                .await?;

            let mut take = this.experiment.lock().await;
            *take = Some(exp.create_experiment);
            drop(take);

            this.consume(ExperimentStateInput::Entered)
                .await
                .map_err(PyErr::from)?;

            Ok(this.clone())
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
        artifact: PartialUploadExperimentArgs,
        data: Vec<u8>,
    ) -> PyResult<&'py PyAny> {
        let this = slf.clone();
        slf.client.runtime().pyfut(py, async move {
            if !matches!(this.state.lock().await.state(), ExperimentStateState::Tests) {
                return Err(Self::ensure_aenter());
            }
            let remote = this.py_experiment().await?;
            let resp = this
                .fail_on(
                    this.client
                        .shared
                        .upload_experiment_artifact(artifact.with_context(remote.id.into()), data)
                        .await,
                )
                .await?;
            #[cfg(debug_assertions)]
            debug!("{:#?}", resp);
            Ok(resp)
        })
    }

    #[getter]
    fn id<'py>(this: PyRef<'py, Self>) -> PyResult<i32> {
        Ok(this.blocking_experiment()?.id)
    }

    #[getter]
    fn experiment<'py>(this: PyRef<'py, Self>) -> PyResult<gql::create_experiment::Experiment> {
        this.blocking_experiment()
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        exc_type: Option<PyObject>,
        exc_value: Option<PyObject>,
        traceback: Option<PyObject>,
    ) -> PyResult<&'py PyAny> {
        let this = self.clone();
        match (exc_type, exc_value, traceback) {
            (None, None, None) => self.client.runtime().pyfut(py, async move {
                this.consume(ExperimentStateInput::WaitClose)
                    .await
                    .map_err(PyErr::from)?;
                Ok(())
            }),
            (ty, value, trace) => {
                // how do we want to handle this
                tracing::error!("{:#?}\n{:#?}\n{:#?}", ty, value, trace);
                self.client.runtime().pyfut(py, async move {
                    this.consume(ExperimentStateInput::Failed)
                        .await
                        .map_err(PyErr::from)?;
                    Ok(())
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use flymodel_graphql::gql::create_experiment::CreateExperimentVariables;

    use crate::py::PythonClient;

    use super::{Experiment, ExperimentStateInput};

    fn base_exp() -> Experiment {
        Experiment::new(
            PythonClient::new("http://localhost:9009".into()).unwrap(),
            CreateExperimentVariables {
                experiment_name: "abc".into(),
                model_version_id: 1,
            },
        )
    }

    #[tokio::test]
    async fn test_state_ok() {
        let exp = base_exp();
        exp.consume(ExperimentStateInput::Started).await.unwrap();
    }
}
