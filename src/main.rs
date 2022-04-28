//! Homebooru, the homebrew booru site
//!
//! A booru is a tagged image board, here is an 
//! [`example`](https://safebooru.org/)
//! 
//! This project aims to write a booru server in rust for personal home use
//!

// use std::env;

pub mod auth;
pub mod api;
pub mod db;
pub mod error;
// pub mod entity;

use db::prelude::*;
use api::prelude::*;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

pub async fn info(id: ActixIdentity) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().body(format!("homebooru.alpha {:?}", id.identity())))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        conn: sea_orm::Database::connect("sqlite:data/db/homebooru.db").await?
    };

    User::create(&state.conn, 
        "hrlou".into(), 
        "hesslewis@gmail.com".into(), 
        "password".into()
    ).await?;

    /*let tags: Vec<(String, String)> = vec![
        ("parody".into(), "genshin impact".into()),
        ("character".into(), "keqing".into()),
        ("artist".into(), "remana".into()),
    ];
    Post::create(&state.conn, tags, post::ActiveModel {
        title: Set(Some("[MANA] 刻晴1".into())),
        source: Set(Some("https://exhentai.org/g/2202032/f8252001bf".into())),
        ..Default::default()
    }).await?;*/

    std::env::set_var(
        "RUST_LOG",
        "homebooru=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    // override with config
    let domain = "localhost".to_string();



    

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&*auth::PRIVATE_KEY)
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(Duration::days(1))
                    .same_site(actix_web::cookie::SameSite::Strict)
                    .secure(false), // this can only be true if you have https
            ))
            .service(web::scope("/api")
                .route("", web::get().to(info))
                .service(
                    web::resource("/auth")
                        .route(web::post().to(api::auth::login))
                        .route(web::delete().to(api::auth::logout))
                        .route(web::get().to(api::auth::get)),
                )
            )
            .service(actix_files::Files::new("/assets", "www/assets/").show_files_listing())
            .service(actix_files::Files::new("/", "www/dist/").index_file("index.html"))
            .default_service(web::to(|| HttpResponse::NotFound()))
            // .default_service(web::to(|| HttpResponse::Found().append_header(("Location", "/")).finish()))

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}