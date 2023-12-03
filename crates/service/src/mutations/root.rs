use async_graphql::Object;

#[derive(Clone, Default)]
pub struct RootMutations;

#[Object]
impl RootMutations {
    pub async fn root_mutation(&self) -> String {
        "root mutation".to_string()
    }
}
