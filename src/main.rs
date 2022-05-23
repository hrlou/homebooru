pub mod api;
pub mod auth;
pub mod db;
pub mod error;

use api::prelude::*;
use db::prelude::*;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

pub async fn info(id: ActixIdentity) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().body(format!("homebooru.alpha {:?}", id.identity())))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let state = AppState {
        conn: sea_orm::Database::connect("sqlite:data/db/homebooru.db").await?,
    };

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
            .service(
                web::scope("/api")
                    .route("", web::get().to(info))
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(api::auth::login))
                            .route(web::delete().to(api::auth::logout))
                            .route(web::get().to(api::auth::get)),
                    )
                    .service(web::resource("/post/{id}").route(web::get().to(api::post::get))),
            )
            .service(actix_files::Files::new("/assets", "www/assets/").show_files_listing())
            .service(actix_files::Files::new("/", "www/app/").index_file("index.html"))
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
