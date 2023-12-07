use anyhow::Context as _;
use async_graphql::{dataloader::Loader, *};
use flymodel::lifecycle::Lifecycle;
use flymodel_entities::{
    db::Database,
    entities::{
        self,
        page::{PageInput, Paginated, PaginatedResult},
    },
};

#[derive(Clone, Default)]
pub struct BucketQueries;

#[Object]
impl BucketQueries {
    async fn bucket<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Option<Vec<i64>>,
        page: Option<PageInput>,
        namespace: Option<Vec<i64>>,
        role: Option<Vec<Lifecycle>>,
    ) -> PaginatedResult<entities::bucket::Model> {
        let db: &Database<entities::bucket::Model> = ctx.data_opt().context("no database")?;

        if let Some(ids) = id {
            let re: Vec<_> = db
                .loader()
                .load(&ids)
                .await?
                .values()
                .map(entities::bucket::Model::to_owned)
                .collect();
            return Ok(Paginated::new(
                (ids.len(), 0),
                1 as usize,
                re.len() as u64,
                re,
            ));
        }

        let page = page.unwrap_or_default();
        db.loader().find_by_namespace(namespace, role, page).await
    }
}
