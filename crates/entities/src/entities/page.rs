use async_graphql::*;

use crate::db::QueryResult;

#[derive(Clone, Debug, PartialEq, Eq, InputObject)]
#[graphql(concrete(name = "Page", params()))]
pub struct PageInput {
    #[graphql(default = 25)]
    pub size: usize,
    pub page: u64,
}

impl Default for PageInput {
    fn default() -> Self {
        Self { size: 25, page: 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(concrete(name = "CurrentPage", params()))]
pub struct PageOutput {
    pub size: usize,
    pub page: u64,
}

impl From<(usize, u64)> for PageOutput {
    fn from((size, page): (usize, u64)) -> Self {
        Self { size, page }
    }
}

impl From<PageInput> for PageOutput {
    fn from(input: PageInput) -> Self {
        Self {
            size: input.size,
            page: input.page + 1,
        }
    }
}

impl PageOutput {
    pub fn new(size: usize, page: u64) -> Self {
        Self { size, page }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(concrete(name = "PaginatedNamespace", params(crate::entities::namespace::Model)))]
#[graphql(concrete(name = "PaginatedBucket", params(crate::entities::bucket::Model)))]
#[graphql(concrete(name = "PaginatedModel", params(crate::entities::model::Model)))]
#[graphql(concrete(
    name = "PaginatedModelArtifact",
    params(crate::entities::model_artifact::Model)
))]
#[graphql(concrete(
    name = "PaginatedModelState",
    params(crate::entities::model_state::Model)
))]
#[graphql(concrete(
    name = "PaginatedModelVersion",
    params(crate::entities::model_version::Model)
))]
#[graphql(concrete(
    name = "PaginatedExperiment",
    params(crate::entities::experiment::Model)
))]
#[graphql(concrete(
    name = "PaginatedExperimentArtifact",
    params(crate::entities::experiment_artifact::Model)
))]
pub struct Paginated<T>
where
    T: OutputType + Send + Clone,
{
    pub page: PageOutput,
    pub total_pages: usize,
    pub total_items: u64,
    pub data: Vec<T>,
}

impl<T> Paginated<T>
where
    T: OutputType + Send + Clone,
{
    pub fn new<P, I>(page: P, total_pages: I, total_items: u64, data: Vec<T>) -> Self
    where
        I: Into<usize>,
        P: Into<PageOutput> + Send + Clone,
    {
        Self {
            page: page.into(),
            total_pages: total_pages.into(),
            total_items,
            data,
        }
    }
}

pub type PaginatedResult<T> = QueryResult<Paginated<T>>;
