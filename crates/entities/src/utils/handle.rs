use flymodel::errs::FlymodelError;
use sea_orm::{DbErr, SqlErr};

use super::sql_errs::parse_column_contraint_violation;

pub fn constraint_or_db_operational(
    constraint: &'static str,
    err: DbErr,
    constraint_str: String,
) -> FlymodelError {
    match &err.sql_err() {
        Some(SqlErr::ForeignKeyConstraintViolation(source)) => {
            match parse_column_contraint_violation(source) {
                Some(index) => {
                    if index == constraint {
                        FlymodelError::ContraintError(constraint_str)
                    } else {
                        FlymodelError::DbOperationError(err)
                    }
                }
                _ => FlymodelError::DbOperationError(err),
            }
        }
        _ => FlymodelError::DbOperationError(err),
    }
}
