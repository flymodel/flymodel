use async_graphql::{Context, Object};

use flymodel_entities::{db::DbLoader, entities};

#[derive(Clone, Default)]
pub struct NamespaceMutations;

#[Object]
impl NamespaceMutations {
    pub async fn create_namespace<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        _name: String,
        _description: Option<String>,
    ) -> Result<String, async_graphql::Error> {
        let _db = DbLoader::<entities::namespace::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?;
        Ok("root mutation".to_string())
    }
}
