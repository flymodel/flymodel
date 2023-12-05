use anyhow::Context as ErrContext;
use async_graphql::{dataloader::Loader, *};
use flymodel_entities::{
    db::Database,
    entities::{
        self,
        enums::Lifecycle,
        page::{PageInput, Paginated},
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
    ) -> anyhow::Result<Paginated<entities::bucket::Model>> {
        let db: &Database<entities::bucket::Model> = ctx.data_opt().context("no database")?;

        if let Some(ids) = id {
            return Ok(Paginated::new(
                (ids.len(), 0),
                1 as usize,
                ids.len() as u64,
                db.loader()
                    .load(&ids)
                    .await?
                    .values()
                    .map(entities::bucket::Model::to_owned)
                    .collect(),
            ));
        }

        let page = page.unwrap_or_default();
        Ok(db.loader().find_by_namespace(namespace, role, page).await?)
    }
}
