use async_graphql::{Context, Object};

use flymodel_entities::{db::DbLoader, entities};

#[derive(Clone, Default)]
pub struct NamespaceMutations;

#[Object]
impl NamespaceMutations {
    pub async fn create_namespace<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
        description: Option<String>,
    ) -> Result<entities::namespace::Model, async_graphql::Error> {
        let db = DbLoader::<entities::namespace::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?
            .loader();
        // user validation here
        db.create_namespace(name, description).await
    }
}
