use async_graphql::{dataloader::Loader, *};
use flymodel::lifecycle::Lifecycle;
use flymodel_entities::{
    db::DbLoader,
    entities::{
        self,
        page::{PageInput, Paginated, PaginatedResult},
    },
};

#[derive(Clone, Default)]
pub struct ModelQueries;

#[Object]
impl ModelQueries {
    async fn model<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Option<Vec<i64>>,
        page: Option<PageInput>,
        name: Option<String>,
        namespace: Option<Vec<i64>>,
        role: Option<Vec<Lifecycle>>,
    ) -> PaginatedResult<entities::model::Model> {
        let db = DbLoader::<entities::model::Model>::with_context(ctx)?;
        if let Some(ids) = id {
            let re: Vec<_> = db
                .loader()
                .load(&ids)
                .await?
                .values()
                .map(|model| model.to_owned())
                .collect();
            return Ok(Paginated::new(
                (ids.len(), 0),
                1 as usize,
                re.len() as u64,
                re,
            ));
        }

        let page = page.unwrap_or_default();
        db.loader()
            .find_by_name_and_namespace(name, namespace, role, page)
            .await
    }
}
