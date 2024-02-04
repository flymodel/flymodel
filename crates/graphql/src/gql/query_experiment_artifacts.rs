use crate::{enums::*, fragments::*, jsvalue, schema};
use flymodel_macros::hybrid_feature_class;

use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct ExperimentArtifactsVariables {
    pub id: i32,
    pub page: Option<Page>,
}

crate::new_for! {
    #[pyo3(signature = (id, page = None))]
    ExperimentArtifactsVariables,
    id: i32,
    page: Option<Page>
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "ExperimentArtifactsVariables")]
pub struct ExperimentArtifacts {
    #[arguments(id: $id)]
    pub experiment: PaginatedExperiment,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cynic(variables = "ExperimentArtifactsVariables")]
pub struct PaginatedExperiment {
    pub data: Vec<Experiment>,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cynic(variables = "ExperimentArtifactsVariables")]
pub struct Experiment {
    #[arguments(page: $page)]
    pub artifacts: PaginatedExperimentArtifact,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(into_wasm_abi))]
pub struct PaginatedExperimentArtifact {
    pub page: CurrentPage,
    pub total_pages: i32,
    pub total_items: i32,
    pub data: Vec<ExperimentArtifact>,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(into_wasm_abi))]
pub struct ExperimentArtifact {
    pub id: i32,
    pub version_id: i32,
    pub name: String,
    pub object: ObjectBlob,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(into_wasm_abi))]
pub struct ObjectBlob {
    pub format: Option<ArchiveFormat>,
    pub encode: Option<ArchiveCompression>,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

jsvalue! {
    ObjectBlob,
    ExperimentArtifact,
    PaginatedExperimentArtifact,
    Experiment,
    PaginatedExperiment
}
