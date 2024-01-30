use async_graphql::{Context, Object};

use flymodel_entities::{db::DbLoader, entities};

#[derive(Clone, Default)]
pub struct ModelMutations;

#[Object]
impl ModelMutations {
    pub async fn create_model<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        namespace: i64,
        name: String,
    ) -> Result<entities::model::Model, async_graphql::Error> {
        let db = DbLoader::<entities::model::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?
            .loader();
        // user validation here
        db.create_model(namespace, name).await
    }

    pub async fn delete_model<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i64,
    ) -> Result<bool, async_graphql::Error> {
        let db = DbLoader::<entities::model::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?
            .loader();
        // user validation here
        db.delete_model(id).await
    }

    pub async fn update_model<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i64,
        name: String,
    ) -> Result<entities::model::Model, async_graphql::Error> {
        let db = DbLoader::<entities::model::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?
            .loader();
        // user validation here
        db.update_model(id, name).await
    }
}
