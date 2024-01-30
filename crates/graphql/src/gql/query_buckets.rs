use crate::{enums::Lifecycle, fragments::*, jsvalue, scalars::*, schema};
use flymodel_macros::hybrid_feature_class;

use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(from_wasm_abi))]
pub struct QueryBucketsVariables {
    pub id: Option<i32>,
    pub namespace: Option<i32>,
    pub page: Option<Page>,
    pub role: Option<Lifecycle>,
}

crate::new_for! {
    #[pyo3(signature = (id = None, namespace = None, page = None, role = None) )]
    QueryBucketsVariables,
    id: Option<i32>,
    namespace: Option<i32>,
    page: Option<Page>,
    role: Option<Lifecycle>,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Query", variables = "QueryBucketsVariables")]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct QueryBuckets {
    #[arguments(page: $page, id: $id, namespace: $namespace, role: $role)]
    pub bucket: PaginatedBucket,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct PaginatedBucket {
    pub page: CurrentPage,
    pub total_pages: i32,
    pub total_items: i32,
    pub data: Vec<Bucket>,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct Bucket {
    pub id: i32,
    pub name: String,
    pub namespace: i32,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub created_at: DateTime,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub last_modified: DateTime,
}

jsvalue! {
    Bucket,
    PaginatedBucket,
    QueryBuckets,
}
