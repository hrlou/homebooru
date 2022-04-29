pub mod auth;
pub mod user;
pub mod post;

pub mod prelude {
    pub use crate::{db::prelude::*, error::ServiceError};
    pub use serde::{Serialize, Deserialize};

    pub use actix_web::{
        dev::Payload,
        middleware::{NormalizePath, TrailingSlash, Logger},
        web, App, HttpServer, Result,
        FromRequest, HttpRequest, HttpResponse,
        Error as ActixError,
    };
    pub use actix_multipart::{Field, Multipart};
    pub use actix_files::{NamedFile};
    pub use actix_identity::{
        CookieIdentityPolicy, IdentityService, 
        Identity as ActixIdentity
    };
    // pub use actix_http::cookie::SameSite;

    pub use time::Duration;
}