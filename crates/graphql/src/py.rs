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
        enums::ArchiveCompression,
        enums::ArchiveFormat,
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
        create_bucket,
        gql::create_bucket::Bucket,
        gql::create_bucket::CreateBucket,
        gql::create_bucket::CreateBucketVariables,
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
        delete_bucket,
        gql::delete_bucket::DeleteBucket,
        gql::delete_bucket::DeleteBucketVariables,
    }

    submodule_model! {
        py,
        m,
        delete_experiment,
        gql::delete_experiment::DeleteExperiment,
        gql::delete_experiment::DeleteExperimentVariables,
    }

    submodule_model! {
        py,
        m,
        delete_model_version,
        gql::delete_model_version::DeleteModelVersion,
        gql::delete_model_version::DeleteModelVersionVariables,
    }

    submodule_model! {
        py,
        m,
        delete_model,
        gql::delete_model::DeleteModel,
        gql::delete_model::DeleteModelVariables,
    }

    submodule_model! {
        py,
        m,
        delete_namespace,
        gql::delete_namespace::DeleteNamespace,
        gql::delete_namespace::DeleteNamespaceVariables,
    }

    submodule_model! {
        py,
        m,
        update_model_version_state,
        gql::update_model_version_state::ModelState,
        gql::update_model_version_state::UpdateModelVersionState,
        gql::update_model_version_state::UpdateModelVersionStateVariables,
    }

    submodule_model! {
        py,
        m,
        update_model,
        gql::update_model::Model,
        gql::update_model::UpdateModel,
        gql::update_model::UpdateModelVariables,
    }

    submodule_model! {
        py,
        m,
        update_namespace,
        gql::update_namespace::Namespace,
        gql::update_namespace::UpdateNamespace,
        gql::update_namespace::UpdateNamespaceVariables,
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

    submodule_model! {
        py,
        m,
        query_experiment,
        gql::query_experiment::Experiment,
        gql::query_experiment::PaginatedExperiment,
        gql::query_experiment::QueryExperiment,
        gql::query_experiment::QueryExperimentVariables,
    }

    Ok(m)
}
