use async_graphql::{Context, Object};

use flymodel::lifecycle::Lifecycle;
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

    #[allow(unused_variables)]
    pub async fn delete_model_version<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i64,
        hard: Option<bool>,
    ) -> Result<bool, async_graphql::Error> {
        let db = DbLoader::<entities::model_version::Model>::with_context(ctx)?.loader();
        // todo: add 'hard' / 'soft' delete
        db.delete_version(id).await
    }

    pub async fn update_model_version_state<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i64,
        state: Lifecycle,
    ) -> Result<entities::model_state::Model, async_graphql::Error> {
        let db = DbLoader::<entities::model_state::Model>::with_context(ctx)?.loader();

        db.update_state(id, state).await
    }
}
