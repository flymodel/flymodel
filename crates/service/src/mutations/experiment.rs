use async_graphql::{Context, Object};

use flymodel_entities::{db::DbLoader, entities};

#[derive(Clone, Default)]
pub struct ExperimentMutations;

#[Object]
impl ExperimentMutations {
    pub async fn create_experiment<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        model_version: i64,
        name: String,
    ) -> Result<entities::experiment::Model, async_graphql::Error> {
        let db = DbLoader::<entities::experiment::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?
            .loader();
        // user validation here
        db.create_experiment(model_version, name).await
    }

    #[allow(unused_variables)]
    pub async fn delete_experiment<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i64,
        hard: Option<bool>,
    ) -> Result<bool, async_graphql::Error> {
        let db = DbLoader::<entities::experiment::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?
            .loader();
        // user validation here
        db.delete_experiment(id).await
    }
}
