use actix_web::{error::ResponseError, HttpResponse};
use derive_more::{Display};
use sea_orm::DbErr;
// derive_more::Error

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

impl From<DbErr> for ServiceError {
    fn from(_: DbErr) -> ServiceError {
        ServiceError::BadRequest("Database Error".into())
    }
}

impl From<actix_web::Error> for ServiceError {
    fn from(_: actix_web::Error) -> ServiceError {
        ServiceError::BadRequest("Server Error".into())
    }
}