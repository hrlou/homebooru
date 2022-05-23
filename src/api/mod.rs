pub mod auth;
pub mod post;
pub mod user;

pub mod prelude {
    pub use crate::{db::prelude::*, error::ServiceError};
    pub use serde::{Deserialize, Serialize};

    pub use actix_files::NamedFile;
    pub use actix_identity::{CookieIdentityPolicy, Identity as ActixIdentity, IdentityService};
    pub use actix_multipart::{Field, Multipart};
    pub use actix_web::{
        dev::Payload,
        middleware::{Logger, NormalizePath, TrailingSlash},
        web, App, Error as ActixError, FromRequest, HttpRequest, HttpResponse, HttpServer, Result,
    };
    // pub use actix_http::cookie::SameSite;

    pub use time::Duration;
}
