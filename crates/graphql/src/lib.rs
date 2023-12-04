#[cynic::schema("flymodel")]
pub mod schema {}

#[cfg(test)]
mod test {
    #![allow(unused_imports, path_statements)]
    use crate::schema;

    // smoke test to ensure we compile our schemas
    #[test]
    fn test_schema() {
        schema::Bucket;
        schema::PaginatedBucket;
        schema::PaginatedNamespace;
    }
}
