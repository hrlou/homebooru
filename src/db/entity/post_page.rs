//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "post_page")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub page: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub post_id: i32,
    pub page_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::media::Entity",
        from = "Column::PageId",
        to = "super::media::Column::Md5",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Media,
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Post,
}

impl Related<super::media::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Media.def()
    }
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
