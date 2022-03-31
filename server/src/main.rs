//! Homebooru, the homebrew booru site
//!
//! A booru is a tagged image board, here is an 
//! [`example`](https://safebooru.org/)
//! 
//! This project aims to write a booru server in rust for personal home use
//!

pub mod config;
pub mod db;
pub mod server;
// pub mod settings;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use db::prelude::*;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config::SERVER.log_level.as_str()));

    /*let pool = SqlitePool::connect("sqlite:data/db/homebooru.db").await?;
    // let mut conn = pool.acquire().await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS todos
        (
            id          INTEGER PRIMARY KEY NOT NULL,
            description TEXT                NOT NULL,
            done        BOOLEAN             NOT NULL DEFAULT 0
        )
    "#)
    .execute(&pool)
    .await?;

    let id = sqlx::query(r#"
        INSERT INTO todos ( description )
        VALUES ( ?1 )
    "#)
    .bind("DEEZ NUTS")
    .execute(&pool)
    .await?
    .last_insert_rowid();

    // println!("{:?}", id);
    
    let rows = sqlx::query(r#"
        SELECT * FROM todos
    "#)
    .fetch_all(&pool)
    .await?;

    for r in rows {
        println!("{:?}: {:?}", r.get::<i64, _>("id"), r.get::<String, _>("description"));
    }

    

    // let id = sqlx::query!(sql)*/

    Ok(())
}