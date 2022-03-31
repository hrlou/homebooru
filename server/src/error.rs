use actix_web::{error::ResponseError, HttpResponse};
use derive_more::{Error, Display};

#[derive(Error, Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Unauthorized")]
    Unauthorized,
    #[display(fmt = "Forbidden")]
    Forbidden,
    #[display(fmt = "Not Found")]
    NotFound,
    #[display(fmt = "Unsupported Content")]
    UnsupportedContent,
    #[display(fmt = "Internal Server Error")]
    InternalError,
    #[display(fmt = "Bad Request")]
    BadRequest,
}

impl ServiceError {
    fn log(err: &impl std::fmt::Debug) { 
        log::warn!("{:?}", err);
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::Unauthorized => HttpResponse::Unauthorized()
                .json("Unauthorized"),
            ServiceError::Forbidden => HttpResponse::Forbidden()
                .json("Forbidden Resource"),
            ServiceError::NotFound => HttpResponse::NotFound()
                .json("Not Found"),
            ServiceError::UnsupportedContent => HttpResponse::UnsupportedMediaType()
                .json("Unsupported Media"),
            ServiceError::InternalError => HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try later"),
            _ => HttpResponse::BadRequest()
                .json("Bad Request"),
        }
    }
}

// ! find a way to simplify this
impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self {
        Self::log(&err);
        Self::InternalError
    }
}

impl From<mongodb::error::Error> for ServiceError {
    fn from(err: mongodb::error::Error) -> Self {
        Self::log(&err);
        Self::InternalError
    }
}

impl From<mongodb::bson::oid::Error> for ServiceError {
    fn from(err: mongodb::bson::oid::Error) -> Self {
        Self::log(&err);
        Self::InternalError
    }
}

/*impl From<image::ImageError> for ServiceError {
    fn from(_: image::ImageError) -> Self {
        Self::InternalError
    }
}*/