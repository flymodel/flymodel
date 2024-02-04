use crate::{enums::*, jsvalue, scalars::*, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true, from_ts = true, rename_from_ts = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
pub struct CreateBucketVariables {
    pub name: String,
    pub namespace_id: i32,
    pub region: Option<String>,
    pub role: Lifecycle,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateBucketVariables")]
#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
pub struct CreateBucket {
    #[arguments(namespace: $namespace_id, name: $name, role: $role, region: $region)]
    pub create_bucket: Bucket,
}

#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[hybrid_feature_class(python = true, into_ts = true, rename_into_ts = true)]
pub struct Bucket {
    pub id: i32,
    pub name: String,
    pub region: String,
    pub namespace: i32,
    pub role: Lifecycle,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub created_at: DateTime,
    #[cfg_attr(feature = "wasm", tsify(type = "string"))]
    pub last_modified: DateTime,
}

jsvalue! {
    Bucket,
    CreateBucket,
}
