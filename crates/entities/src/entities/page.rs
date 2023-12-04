use async_graphql::*;

#[derive(Clone, Debug, PartialEq, Eq, InputObject)]
#[graphql(concrete(name = "Page", params()))]
pub struct PageInput {
    #[graphql(default = 25)]
    size: usize,
    page: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(concrete(name = "CurrentPage", params()))]
pub struct PageOutput {
    size: usize,
    page: usize,
}

impl From<(usize, usize)> for PageOutput {
    fn from((size, page): (usize, usize)) -> Self {
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
    pub fn new(size: usize, page: usize) -> Self {
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
    data: Vec<T>,
}

impl<T> Paginated<T>
where
    T: OutputType + Send + Clone,
{
    pub fn new<P>(page: P, total_pages: usize, data: Vec<T>) -> Self
    where
        P: Into<PageOutput> + Send + Clone,
    {
        Self {
            page: page.into(),
            total_pages,
            data,
        }
    }
}
