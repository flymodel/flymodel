use async_graphql::{dataloader::Loader, *};
use flymodel_entities::{
    db::DbLoader,
    entities::{
        self,
        page::{PageInput, Paginated, PaginatedResult},
    },
};

#[derive(Clone, Default)]
pub struct NamespaceQueries;

#[Object]
impl NamespaceQueries {
    async fn namespace<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Option<Vec<i64>>,
        name: Option<String>,
        page: Option<PageInput>,
    ) -> PaginatedResult<entities::namespace::Model> {
        let db = DbLoader::<entities::namespace::Model>::with_context(ctx)
            .map_err(|err| err.into_graphql_error())?;
        if let Some(id) = id {
            let re: Vec<_> = db
                .loader()
                .load(&id)
                .await?
                .values()
                .map(|ns| ns.to_owned())
                .collect();
            return Ok(Paginated::new(
                (id.len(), 0),
                1 as usize,
                re.len() as u64,
                re,
            ));
        }
        db.loader()
            .bulk_paginated_namespaces(name, page.unwrap_or_default())
            .await
    }
}
