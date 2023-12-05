#[cynic::schema("flymodel")]
pub mod schema {}

#[allow(dead_code)]
pub mod queries {
    use super::schema;

    use chrono::{DateTime, Utc};
    use cynic::impl_scalar;
    impl_scalar!(DateTime<Utc>, schema::DateTime);

    #[derive(cynic::QueryVariables)]
    struct NamespaceVariables {
        pub id: Option<Vec<cynic::Id>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Namespace", variables = "NamespaceVariables")]
    struct Namespace {
        pub id: Option<i32>,
        pub name: Option<String>,
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports, path_statements)]
    use crate::schema;

    // smoke test to ensure we compile our schemas
    #[test]
    fn test_schema() {
        schema::Bucket;
        schema::PaginatedBucket;
        schema::PaginatedNamespaces;
    }
}
