use crate::{artifacts, client::Result, trace::init_subscriber, Client};
use flymodel_graphql::gql::*;
use pyo3::prelude::*;
use std::{
    future::Future,
    sync::{Arc, Once},
};

pub mod experiment;

static INIT: Once = Once::new();

tokio::task_local! {
    static TASKS: once_cell::unsync::OnceCell<pyo3_asyncio::TaskLocals>;
}

#[pymodule]
fn client(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PythonClient>()?;
    m.add_class::<experiment::Experiment>()?;
    m.add_class::<artifacts::UploadExperimentArgs>()?;
    m.add_class::<artifacts::PartialUploadExperimentArgs>()?;
    m.add_class::<artifacts::UploadModelVersionArgs>()?;
    m.add_class::<artifacts::PartialUploadModelVersionArgs>()?;
    m.add_class::<artifacts::UploadRequestParams>()?;

    m.add_submodule(flymodel_graphql::py::submodule(py)?)?;
    Ok(())
}

pub(crate) struct Runtime {
    _rt: Option<tokio::runtime::Runtime>,
    handle: tokio::runtime::Handle,
}

impl Clone for Runtime {
    fn clone(&self) -> Self {
        Self::with_current(self.handle.clone())
    }
}

impl Runtime {
    fn new() -> Self {
        INIT.call_once(|| init_subscriber());
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => Self::with_current(handle),
            Err(_) => Self::new_runtime(),
        }
    }

    fn new_runtime() -> Self {
        tracing::info!(name: "initialization", "creating new runtime");
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("ok");
        let _guard = rt.enter();
        let handle = rt.handle().clone();
        Runtime {
            handle,
            // we need to keep the runtime alive until the end of the program
            _rt: Some(rt),
        }
    }

    fn with_current(handle: tokio::runtime::Handle) -> Self {
        tracing::info!(name: "initialization", "reusing existing runtime");
        Self { handle, _rt: None }
    }

    pub(crate) fn pyfut<
        'a,
        F: Future<Output = PyResult<T>> + Send + 'static,
        T: IntoPy<PyObject>,
    >(
        &self,
        py: Python<'a>,
        fut: F,
    ) -> PyResult<&'a PyAny> {
        // we are going to explicitly control our guard handle
        // and ensure that we do not drop the runtime until the future:
        //  - completes
        //  - rejects
        //  - cancels
        let handle = self.handle.enter();
        pyo3_asyncio::generic::future_into_py::<Runtime, _, T>(py, fut).map(|out| {
            drop(handle);
            out
        })
    }
}

impl pyo3_asyncio::generic::Runtime for Runtime {
    type JoinError = tokio::task::JoinError;
    type JoinHandle = tokio::task::JoinHandle<()>;

    fn spawn<F>(fut: F) -> Self::JoinHandle
    where
        F: Future<Output = ()> + Send + 'static,
    {
        tokio::runtime::Handle::current().spawn(async move {
            fut.await;
        })
    }
}

impl pyo3_asyncio::generic::ContextExt for Runtime {
    fn scope<F, R>(
        locals: pyo3_asyncio::TaskLocals,
        fut: F,
    ) -> std::pin::Pin<Box<dyn Future<Output = R> + Send>>
    where
        F: Future<Output = R> + Send + 'static,
    {
        let cell = once_cell::unsync::OnceCell::new();
        cell.set(locals).unwrap();

        Box::pin(TASKS.scope(cell, fut))
    }

    fn get_task_locals() -> Option<pyo3_asyncio::TaskLocals> {
        match TASKS.try_with(|c| c.get().map(|locals| locals.clone())) {
            Ok(locals) => locals,
            Err(_) => None,
        }
    }
}

#[derive(Clone)]
#[pyo3::prelude::pyclass(name = "Client")]
pub struct PythonClient {
    shared: Arc<Client>,
    rt: Runtime,
}

impl AsRef<Client> for PythonClient {
    fn as_ref(&self) -> &Client {
        &self.shared
    }
}

macro_rules! impl_associated_futures {
    (
        $(
            pub async fn $name: ident (&self, $($arg: ident: $typ: ty), + $(,)?) -> $ret: ty
        ), + $(,)?
    ) => {
            #[pymethods]
            impl PythonClient {
                #[new]
                pub fn new(
                        base_url: String,
                    ) -> Result<Self> {
                    Ok(PythonClient{
                        shared: Arc::new(Client::new_common(&base_url)?),
                        rt: Runtime::new(),
                    })
                }

                $(
                    pub fn $name<'py>(&self, py: Python<'py>, $($arg: $typ)*) -> PyResult<&'py PyAny> {
                        let client = self.shared.clone();
                        let handle = self.rt.handle.clone();
                        self.rt.pyfut(py, async move {
                            let res = client.$name($($arg)*).await?;
                            Ok(res)
                        }).map(|re| {
                            drop(handle);
                            re
                        })
                    }
                )*
            }
    };
}

impl PythonClient {
    pub(crate) fn runtime(&self) -> &Runtime {
        &self.rt
    }
}

impl_associated_futures! {
    pub async fn create_bucket(&self, bucket: create_bucket::CreateBucketVariables) -> Result<create_bucket::CreateBucket>,

    pub async fn delete_bucket(&self, bucket: delete_bucket::DeleteBucketVariables) -> Result<delete_bucket::DeleteBucket>,

    pub async fn create_namespace(&self, namespace: create_namespace::CreateNamespaceVariables) -> Result<create_namespace::CreateNamespace>,

    pub async fn delete_namespace(&self, namespace: delete_namespace::DeleteNamespaceVariables) -> Result<delete_namespace::DeleteNamespace> ,

    pub async fn update_namespace(&self, namespace: update_namespace::UpdateNamespaceVariables) -> Result<update_namespace::UpdateNamespace>,

    pub async fn create_model(
        &self,
        model: create_model::CreateModelVariables,
    ) -> Result<create_model::CreateModel>,

    pub async fn delete_model(&self, model: delete_model::DeleteModelVariables) -> Result<delete_model::DeleteModel>,

    pub async fn update_model(&self, model: update_model::UpdateModelVariables) -> Result<update_model::UpdateModel>,

    pub async fn create_model_version(
        &self,
        version: create_model_version::CreateModelVersionVariables,
    ) -> Result<create_model_version::CreateModelVersion> ,

    pub async fn delete_model_version(
        &self,
        version: delete_model_version::DeleteModelVersionVariables,
    ) -> Result<delete_model_version::DeleteModelVersion>,

    pub async fn update_model_version_state(&self, state: update_model_version_state::UpdateModelVersionStateVariables) -> Result<update_model_version_state::UpdateModelVersionState>,

    pub async fn create_experiment(
        &self,
        experiment: create_experiment::CreateExperimentVariables,
    ) -> Result<create_experiment::CreateExperiment>,

    pub async fn delete_experiment(
        &self,
        experiment: delete_experiment::DeleteExperimentVariables,
    ) -> Result<delete_experiment::DeleteExperiment>,

    pub async fn query_namespaces(&self, vars: query_namespaces::QueryNamespacesVariables) -> Result<query_namespaces::QueryNamespaces> ,

    pub async fn query_buckets(&self, vars: query_buckets::QueryBucketsVariables) -> Result<query_buckets::QueryBuckets>,

    pub async fn query_namespace_models(
        &self,
        vars: query_models::NamespaceModelsVariables,
    ) -> Result<query_models::NamespaceModels>,

    pub async fn query_experiment(&self, vars: query_experiment::QueryExperimentVariables) -> Result<query_experiment::QueryExperiment>,
}
