use async_graphql::{dataloader::Loader, *};
use flymodel_entities::{
    db::DbLoader,
    entities::{
        self,
        enums::Lifecycle,
        page::{PageInput, Paginated},
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
    ) -> anyhow::Result<Paginated<entities::model::Model>> {
        let db = DbLoader::<entities::model::Model>::context(ctx)?;
        if let Some(ids) = id {
            return Ok(Paginated::new(
                (ids.len(), 0),
                1 as usize,
                ids.len() as u64,
                db.loader()
                    .load(&ids)
                    .await?
                    .values()
                    .map(|model| model.to_owned())
                    .collect(),
            ));
        }

        let page = page.unwrap_or_default();
        Ok(db
            .loader()
            .find_by_name_and_namespace(name, namespace, role, page)
            .await?)
    }
}
