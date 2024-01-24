use anyhow::Context as _;
use async_graphql::{dataloader::Loader, *};

use flymodel_entities::{
    db::Database,
    entities::{
        self,
        page::{PageInput, Paginated, PaginatedResult},
    },
};

#[derive(Clone, Default)]
pub struct ExperimentQueries;

#[Object]
impl ExperimentQueries {
    async fn experiment<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Option<Vec<i64>>,
        model_id: Option<i64>,
        page: Option<PageInput>,
        name: Option<String>,
    ) -> PaginatedResult<entities::experiment::Model> {
        let db: &Database<entities::experiment::Model> = ctx.data_opt().context("no database")?;

        if let Some(ids) = id {
            let re: Vec<_> = db
                .loader()
                .load(&ids)
                .await?
                .values()
                .map(entities::experiment::Model::to_owned)
                .collect();
            return Ok(Paginated::new(
                (ids.len(), 0),
                1 as usize,
                re.len() as u64,
                re,
            ));
        }

        db.loader()
            .bulk_paginated_experiments(name, model_id, page.unwrap_or_default())
            .await
    }
}
