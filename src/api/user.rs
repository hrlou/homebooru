use crate::api::prelude::*;
use tokio::{
    macros::support::{Future, Pin},
    sync::RwLock,
};

impl FromRequest for user::Model {
    type Error = ServiceError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let id = ActixIdentity::from_request(req, pl).into_inner();
        let id = match id {
            Ok(id) => id,
            _ => return Box::pin(async { Err(Self::Error::BadRequest("unauthorized".into())) }),
        };
        Box::pin(async move {
            if let Some(id) = id.identity() {
                let user: user::Model = serde_json::from_str(id.as_str()).unwrap();
                return Ok(user);
            };
            Err(Self::Error::BadRequest("unauthorized".into()))
        })
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}
