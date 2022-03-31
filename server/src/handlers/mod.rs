pub use crate::{models::*, server::*, db::*};

pub mod media;

pub async fn multipart_fields(mut payload: Multipart) -> Result<Vec<Field>, ServiceError> {
    use futures::{TryStreamExt};

    let mut v: Vec<Field> = vec![];
    while let Ok(Some(field)) = payload.try_next().await {
        v.push(field);
    }
    Ok(v)
}
