pub use serde::{de::DeserializeOwned, Serialize, Deserialize};
pub use crate::{db::*, server::*, error::*, settings::*};
// use nanoid::nanoid;

pub mod media;
// pub mod tag;
// pub mod post;

/// allow us to have some basic polymorphism with the documents
pub trait Model: Sync + Send + Unpin + Serialize + DeserializeOwned {
    type ID: Into<Bson> + Clone;
    const COLLECTION: &'static str;
}

pub type Media = media::Media;