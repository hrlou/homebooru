use crate::error::ServiceError;
use argon2::{self, Config};
use rand::Rng;

lazy_static::lazy_static! {
    pub static ref PRIVATE_KEY: [u8; 32] = rand::thread_rng().gen();
}

pub fn hash(password: &str) -> Result<String, ServiceError> {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded(hash, password.as_bytes()).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}