use crate::db::Database;
use anyhow::Context as ErrContext;
use async_graphql::{dataloader::Loader, *};
use flymodel_entities::{
    entities,
    entities::page::{PageInput, PageOutput, Paginated},
};

#[derive(Clone, Default)]
pub struct NamespaceQueries;

#[Object]
impl NamespaceQueries {
    async fn namespace<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Option<Vec<i32>>,
        page: Option<PageInput>,
    ) -> anyhow::Result<Paginated<entities::namespace::Model>> {
        let db: &Database = ctx.data_opt().context("no database")?;

        if let Some(id) = id {
            return Ok(Paginated::new(
                (id.len(), 0),
                1,
                db.loader()
                    .load(&id)
                    .await?
                    .iter()
                    .map(|(_, ns)| ns.to_owned())
                    .collect(),
            ));
        }

        Ok(Paginated::new((0, 0), 0, vec![]))
    }
}
