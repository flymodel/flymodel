use crate::db::Database;
use anyhow::Context as ErrContext;
use async_graphql::{dataloader::Loader, *};
use flymodel_entities::{
    entities,
    entities::page::{PageInput, Paginated},
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
        let db: &Database<entities::namespace::Model> = ctx.data_opt().context("no database")?;

        if let Some(id) = id {
            return Ok(Paginated::new(
                (id.len(), 0),
                1 as usize,
                id.len() as u64,
                db.loader()
                    .load(&id)
                    .await?
                    .values()
                    .map(|ns| ns.to_owned())
                    .collect(),
            ));
        }
        let page = page.unwrap_or_default();
        Ok(db.loader().bulk_paginated_namespaces(page).await?)
    }
}
