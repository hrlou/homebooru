pub mod entity;

pub mod prelude {
    pub use crate::{AppState, db};
    pub use super::entity::{
        prelude::*,
        post, tag, user, post_tag
    };
    pub use sqlx::{
        FromRow, Row, 
        sqlite::{
            self, SqlitePool, SqlitePoolOptions, SqliteRow
        },
    };
    pub use sea_orm::{DatabaseConnection, DbErr, entity::*, query::*};
}

use prelude::*;

use crate::api::prelude::ServiceError;

// this is just for development, since I'm regenerating the entities
impl Related<super::tag::Entity> for super::post::Entity {
    fn to() -> RelationDef {
        super::post_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::post_tag::Relation::Post.def().rev())
    }
}

impl Related<super::post::Entity> for super::tag::Entity {
    fn to() -> RelationDef {
        super::post_tag::Relation::Post.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::post_tag::Relation::Tag.def().rev())
    }
}

impl Tag {
    pub async fn create(conn: &DatabaseConnection, tag: (String, String)) -> Result<i32, Box<dyn std::error::Error>>  {
        let (kind, name) = tag;
        let id = Tag::find()
            .filter(tag::Column::Kind.eq(kind.clone()))
            .filter(tag::Column::Name.eq(name.clone()))
            .one(conn).await?;
        let id = match id {
            Some(model) => model.id,
            None => {
                Tag::insert(tag::ActiveModel {
                    kind: Set(kind), 
                    name: Set(name),
                    ..Default::default()
                }).exec(conn).await?.last_insert_id
            }
        };
        Ok(id)
    }
}

impl Post {
    pub async fn create(conn: &DatabaseConnection, tags: Vec<(String, String)>, post: post::ActiveModel) -> Result<i32, Box<dyn std::error::Error>> {
        let id = Post::insert(post).exec(conn).await?.last_insert_id;
        for t in tags {
            Post::add_tag(conn, id, t).await?;
        }
        Ok(id)
    }

    pub async fn get(conn: &DatabaseConnection, post: i32) -> Result<(post::Model, Vec<tag::Model>), ServiceError> {
        if let Some(post) = Post::find_by_id(post)
            .one(conn)
            .await?
        {
            let tags = post.find_related(Tag).all(conn).await?;
            return Ok((post, tags));
        }
        Err(ServiceError::BadRequest("No such post".into()))
    }
    
    pub async fn add_tag(conn: &DatabaseConnection, post: i32, tag: (String, String)) -> Result<(), Box<dyn std::error::Error>> {
        let id = Tag::create(conn, tag).await?;
        PostTag::insert(post_tag::ActiveModel {
            post_id: Set(post.into()),
            tag_id: Set(id.into()),
        }).exec(conn).await?;
        Ok(())
    }    
}

impl User {
    pub async fn create(conn: &DatabaseConnection, username: String, email: String, password: String) -> Result<i32, ServiceError> {
        let hash = crate::auth::hash(password.as_str())?;
        let user = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            hash: Set(hash),
            ..Default::default()
        };
        match User::insert(user).exec(conn).await {
            Ok(user) => {
                let id = user.last_insert_id;
                Ok(id)
            }
            Err(e) => {
                log::error!("{:?}", e);
                Err(ServiceError::BadRequest("cannot create user".into()))
            }
        }   
    }
}
/*
pub async fn post_remove_tag(conn: &DatabaseConnection, post: i32, tag: (String, String)) -> Result<(), Box<dyn std::error::Error>> {
    // delete tag if it's only on this post
    Ok(())
}
*/