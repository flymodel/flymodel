use async_graphql::*;

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
pub struct Paginated<T>
where
    T: OutputType + Send + Clone,
{
    page: PageOutput,
    total_pages: usize,
    total_items: u64,
    data: Vec<T>,
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
