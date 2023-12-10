use async_graphql::{Context, Object};

use flymodel::lifecycle::Lifecycle;
use flymodel_entities::{db::DbLoader, entities};

#[derive(Clone, Default)]
pub struct BucketMutations;

#[Object]
impl BucketMutations {
    pub async fn create_bucket<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        namespace: i64,
        name: String,
        region: Option<String>,
        role: Lifecycle,
    ) -> Result<entities::bucket::Model, async_graphql::Error> {
        let db = DbLoader::<entities::bucket::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?
            .loader();
        // user validation here
        db.create_bucket(namespace, name, region, role).await
    }
}
