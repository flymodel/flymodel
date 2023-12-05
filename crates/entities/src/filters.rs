use sea_orm::entity::prelude::*;
use sea_query::{Func, IntoColumnRef};

pub fn filter_like<T, Col>(sel: Select<T>, col: Col, name: String) -> Select<T>
where
    T: sea_orm::EntityTrait,
    Col: IntoColumnRef + ColumnTrait,
{
    if !name.is_empty() {
        sel.filter(Expr::expr(Func::lower(Expr::col(col))).like(&format!(
            "%{}%",
            &name.to_lowercase().trim().replace(" ", "%")
        )))
    } else {
        sel
    }
}
#[cfg(test)]
mod test {
    use sea_orm::{DbBackend, EntityTrait, QueryTrait};

    use crate::filters::filter_like;

    #[test]
    fn test_filter_like() {
        let sel = crate::entities::namespace::Entity::find();
        let query = filter_like(
            sel,
            crate::entities::namespace::Column::Name,
            "test".to_string(),
        )
        .build(DbBackend::Postgres)
        .to_string();
        assert_eq!(
            query,
            "SELECT \"namespace\".\"id\", \"namespace\".\"name\", \"namespace\".\"description\", \"namespace\".\"created_at\", \"namespace\".\"last_modified\" FROM \"namespace\" WHERE LOWER(\"name\") LIKE '%test%'"
        );
    }
}
