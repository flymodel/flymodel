use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    SimpleObject,
    serde::Serialize,
    serde::Deserialize,
)]
#[graphql(name = "ModelVersionTag")]
#[sea_orm(table_name = "model_version_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub version_id: i64,
    #[sea_orm(column_type = "Text")]
    pub tag: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model_version::Entity",
        from = "Column::VersionId",
        to = "super::model_version::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ModelVersion,
}

impl Related<super::model_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelVersion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
