#[macro_export]
macro_rules! bulk_loader {
    ($model: ty) => {
        #[async_trait::async_trait]
        impl Loader<i32> for DbLoader<$model> {
            type Value = $model;
            type Error = std::sync::Arc<DbErr>;

            async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, $model>, Self::Error> {
                Entity::find()
                    .filter(Column::Id.is_in(keys.iter().map(|it| *it as i32).collect::<Vec<_>>()))
                    .all(&self.db)
                    .await
                    .map(|re| {
                        std::collections::HashMap::from_iter(
                            re.iter().map(|it| (it.id as i32, it.to_owned())),
                        )
                    })
                    .map_err(std::sync::Arc::new)
            }
        }
    };
}

#[macro_export]
macro_rules! paginated {
    ($model: ty, $entity: ty) => {
        impl $crate::db::DbLoader<$model> {
            pub async fn load_paginated(
                &self,
                sel: Select<$entity>,
                page: $crate::entities::page::PageInput,
            ) -> Result<$crate::entities::page::Paginated<$model>, std::sync::Arc<DbErr>> {
                let selector = sel.paginate(&self.db, page.size as u64);
                let items_pg = selector.num_items_and_pages().await?;
                selector
                    .fetch_page(page.page as u64)
                    .await
                    .map_err(Arc::new)
                    .map(|it| {
                        Paginated::new(
                            page,
                            items_pg.number_of_pages as usize,
                            items_pg.number_of_items,
                            it,
                        )
                    })
            }
        }
    };
}
