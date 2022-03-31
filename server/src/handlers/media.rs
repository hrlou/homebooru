use std::vec;

use crate::handlers::*;
// use actix_web::http::header::HttpDate;
// use futures::{TryStreamExt};

pub async fn get(db: web::Data<Database>, path: web::Path<String>) -> Result<NamedFile, ServiceError> {
    use std::str::FromStr;

    let id = path.into_inner();
    let id = oid::ObjectId::from_str(&id)?;
    let doc = db.get_by_id::<Media>(id).await?;
    let path = doc
        .path()
        .join(doc.name);
    let file = NamedFile::open(path)?;
    Ok(file)
}

pub async fn post(db: web::Data<Database>, payload: Multipart) -> Result<HttpResponse, ServiceError> {
    let fields = multipart_fields(payload).await?;
    let mut v: Vec<Media> = vec![];
    for mut f in fields {
        let media = Media::from_field(&f)?;
        media.write_field(&mut f).await?;
        db.insert(&media).await?;
        v.push(media);
    }
    Ok(HttpResponse::Ok()
        .json(v))
}