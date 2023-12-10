use async_graphql::{Context, Object};

use flymodel_entities::{db::DbLoader, entities};

#[derive(Clone, Default)]
pub struct ModelVersionMutations;

#[Object]
impl ModelVersionMutations {
    pub async fn create_model_version<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        model: i64,
        name: String,
    ) -> Result<entities::model_version::Model, async_graphql::Error> {
        let db = DbLoader::<entities::model_version::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?
            .loader();
        // user validation here
        db.create_version(model, name).await
    }
}
