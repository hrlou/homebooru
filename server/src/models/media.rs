use std::path::PathBuf;

use crate::models::*;
use mime::Mime;
use futures::{StreamExt};
use tokio::{fs, io::AsyncWriteExt};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    pub name: String,
    pub mime: String,
    // pub created_at: chrono::NaiveDateTime,
}

impl Model for Media {
    type ID = oid::ObjectId;
    const COLLECTION: &'static str = "media";
}

// handling fields
impl Media {
    pub fn from_field(field: &Field) -> Result<Self, ServiceError> {
        /*let disposition = field
            .content_disposition()
            .ok_or::<ServiceError>(ServiceError::BadRequest)?;*/
        let disposition = field.content_disposition();
        let name = disposition
            .get_filename()
            .ok_or::<ServiceError>(ServiceError::BadRequest)?;
        let name = String::from(name);
        let mime = field.content_type();
        Media::init(name, mime.clone())
    }

    pub async fn write_field(&self, field: &mut Field) -> Result<(), ServiceError> {
        fs::create_dir_all(self.path()).await?;
        let path = self.path()
            .join(&self.name);
        let mut fs = fs::File::create(&path).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            fs.write_all(&data).await?;  
        };
        Ok(())
    }
}

impl Media {
    pub fn init(name: String, mime: Mime) -> Result<Self, ServiceError> {
        let (name, mime) = Media::sanitise_fields(name, mime)?;
        let media = Media {
            // place holder id
            id: oid::ObjectId::new(),
            name: name,
            mime: mime.to_string(),
        };
        Ok(media)

    }

    pub fn sanitise_fields(name: String, mime: Mime) -> Result<(String, Mime), ServiceError> {
        let mime = match (mime.type_(), mime.subtype()) {
            (mime::IMAGE, _) => mime,
            _ => return Err(ServiceError::UnsupportedContent),
        };
        // add Media name sanitisation
        Ok((name, mime))
    }

    #[inline]
    pub fn path(&self) -> PathBuf {
        PathBuf::new()
            .join(&SETTINGS.path)
            .join(Self::COLLECTION)
            .join(self.id.to_hex())
    }
}