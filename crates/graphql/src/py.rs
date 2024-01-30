use pyo3::prelude::*;

use crate::{enums, fragments, gql};

// pub fn create_experiment(m: &PyModule) -> PyResult<()> {
//     m.add_class::<gql::create_experiment::CreateExperiment>();

//     Ok(())
// }

macro_rules! submodule_model {
    (
        $py: ident,
        $module: ident,
        $name: ident,
        $(
            $target:path,
        )*
    ) => {
        let child = PyModule::new($py, stringify!($name))?;
        fn $name(m: &PyModule) -> PyResult<()> {
            $(
                m.add_class::<$target>()?;
            )*
            Ok(())
        }
        $name(&child)?;
        $module.add_submodule(&child)?;
    };
}

pub fn submodule<'py>(py: Python<'py>) -> PyResult<&'py PyModule> {
    let m = PyModule::new(py, "models")?;

    submodule_model! {
        py,
        m,
        enums,
        enums::Lifecycle,
    }

    submodule_model! {
        py,
        m,
        common,
        fragments::Page,
        fragments::CurrentPage,
    }

    submodule_model! {
        py,
        m,
        create_namespace,
        gql::create_namespace::Namespace,
        gql::create_namespace::CreateNamespace,
        gql::create_namespace::CreateNamespaceVariables,
    }

    submodule_model! {
        py,
        m,
        create_experiment,
        gql::create_experiment::Experiment,
        gql::create_experiment::CreateExperiment,
        gql::create_experiment::CreateExperimentVariables,
    }

    submodule_model! {
        py,
        m,
        create_model_version,
        gql::create_model_version::ModelVersion,
        gql::create_model_version::CreateModelVersion,
        gql::create_model_version::CreateModelVersionVariables,
    }

    submodule_model! {
        py,
        m,
        create_model,
        gql::create_model::Model,
        gql::create_model::CreateModel,
        gql::create_model::CreateModelVariables,
    }

    submodule_model! {
        py,
        m,
        query_buckets,
        gql::query_buckets::Bucket,
        gql::query_buckets::PaginatedBucket,
        gql::query_buckets::QueryBuckets,
        gql::query_buckets::QueryBucketsVariables,
    }

    submodule_model! {
        py,
        m,
        query_models,
        gql::query_models::Model,
        gql::query_models::PaginatedModel,
        gql::query_models::NamespaceModels,
        gql::query_models::NamespaceModelsVariables,
    }

    submodule_model! {
        py,
        m,
        query_namespaces,
        gql::query_namespaces::Namespace,
        gql::query_namespaces::PaginatedNamespace,
        gql::query_namespaces::QueryNamespaces,
        gql::query_namespaces::QueryNamespacesVariables,
    }
    Ok(m)
}
