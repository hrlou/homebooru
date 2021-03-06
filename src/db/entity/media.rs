//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "media")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub md5: String,
    pub name: String,
    pub mime: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post_page::Entity")]
    PostPage,
}

impl Related<super::post_page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostPage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
