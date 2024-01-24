use cynic::impl_scalar;

use crate::schema;

pub type DateTime = chrono::DateTime<chrono::Utc>;

impl_scalar!(DateTime, schema::DateTime);
