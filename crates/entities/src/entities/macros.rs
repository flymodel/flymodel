#[macro_export]
macro_rules! bulk_loader {
    ($model: ty) => {
        impl async_graphql::dataloader::Loader<i64> for crate::db::DbLoader<$model> {
            type Value = $model;
            type Error = std::sync::Arc<DbErr>;

            fn load(
                &self,
                keys: &[i64],
            ) -> impl futures_util::Future<
                Output = Result<std::collections::HashMap<i64, Self::Value>, Self::Error>,
            > + Send {
                async move {
                    Entity::find()
                        .filter(
                            Column::Id.is_in(keys.iter().map(|it| *it as i64).collect::<Vec<_>>()),
                        )
                        .all(&self.db)
                        .await
                        .map(|re| {
                            std::collections::HashMap::from_iter(
                                re.iter().map(|it| (it.id as i64, it.to_owned())),
                            )
                        })
                        .map_err(std::sync::Arc::new)
                }
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
            ) -> $crate::entities::page::PaginatedResult<$model> {
                let selector = sel.paginate(&self.db, page.size as u64);
                let items_pg = selector.num_items_and_pages().await.map_err(|it| {
                    flymodel::errs::FlymodelError::DbOperationError(it).into_graphql_error()
                })?;
                selector
                    .fetch_page(page.page as u64)
                    .await
                    .map_err(|e| {
                        flymodel::errs::FlymodelError::DbOperationError(e).into_graphql_error()
                    })
                    .map(|it| {
                        $crate::entities::page::Paginated::new(
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

#[macro_export]
macro_rules! tags_meta {
    () => {
        pub async fn meta(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> $crate::db::QueryResult<super::namespace_tag::Model> {
            let loader: &$crate::db::DbLoader<super::namespace_tag::Model> =
                $crate::db::DbLoader::with_context(ctx)?.loader();
            super::namespace_tag::Entity::find()
                .filter(super::namespace_tag::Column::Id.eq(self.tag.clone()))
                .one(&loader.db)
                .await
                .map_err(|err| {
                    flymodel::errs::FlymodelError::DbLoaderError(std::sync::Arc::new(err))
                })?
                .ok_or_else(|| {
                    flymodel::errs::FlymodelError::NonDeterministicError(format!(
                        "must have a super tag: {}",
                        self.id
                    ))
                    .into_graphql_error()
                })
        }
    };
}

#[macro_export]
macro_rules! tags_of {
    ($mod: ident, $col: ident) => {
        pub async fn tags(
            &self,
            ctx: &async_graphql::Context<'_>,
            page: Option<PageInput>,
        ) -> PaginatedResult<super::$mod::Model> {
            let loader: &DbLoader<super::$mod::Model> = DbLoader::with_context(ctx)?.loader();
            loader
                .load_paginated(
                    super::$mod::Entity::find().filter(super::$mod::Column::$col.eq(self.id)),
                    page.unwrap_or_default(),
                )
                .await
        }
    };
}
