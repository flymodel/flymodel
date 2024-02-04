use crate::{fragments::*, jsvalue, scalars::*, schema};
use flymodel_macros::hybrid_feature_class;

use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct QueryExperimentVariables {
    pub id: Option<i32>,
    pub model_id: Option<i32>,
    pub name: Option<String>,
    pub page: Option<Page>,
}

crate::new_for! {
    #[pyo3(signature = (id = None, model_id = None, name = None, page = None) )]
    QueryExperimentVariables,
    id: Option<i32>,
    model_id: Option<i32>,
    name: Option<String>,
    page: Option<Page>,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "QueryExperimentVariables")]
pub struct QueryExperiment {
    #[arguments(modelId: $model_id, name: $name, id: $id, page: $page)]
    pub experiment: PaginatedExperiment,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
pub struct PaginatedExperiment {
    pub total_pages: i32,
    pub total_items: i32,
    pub page: CurrentPage,
    pub data: Vec<Experiment>,
}

#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
pub struct Experiment {
    pub id: i32,
    pub name: String,
    pub version_id: i32,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub created_at: DateTime,
}

jsvalue! {
    Experiment,
    PaginatedExperiment,
    QueryExperiment,
}
