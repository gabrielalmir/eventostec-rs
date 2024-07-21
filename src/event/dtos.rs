use std::io;

use axum::extract::Multipart;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateEventDTO {
    pub title: String,
    pub description: String,
    pub date: i64,
    pub remote: bool,
    pub event_url: String,
    pub image: Vec<u8>,
    pub original_filename: String,
}

impl CreateEventDTO {
    pub fn new(
        title: String,
        description: String,
        date: i64,
        remote: bool,
        event_url: String,
        image: Vec<u8>,
        original_filename: String,
    ) -> Self {
        Self {
            title,
            description,
            date,
            remote,
            event_url,
            image,
            original_filename,
        }
    }

    pub async fn from_multipart(multipart: &mut Multipart) -> Result<Self, io::Error> {
        let mut title = None;
        let mut description = None;
        let mut date = None;
        let mut remote = None;
        let mut event_url = None;
        let mut image = None;

        let mut original_filename = None;

        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();

            match name.as_str() {
                "title" => title = Some(field.text().await.unwrap()),
                "description" => description = Some(field.text().await.unwrap()),
                "date" => date = Some(field.text().await.unwrap()),
                "remote" => remote = Some(field.text().await.unwrap()),
                "event_url" => event_url = Some(field.text().await.unwrap()),
                "image" => {
                    if let Some(filename) = field.file_name() {
                        original_filename = Some(filename.to_string());
                        image = Some(field.bytes().await.unwrap());
                    }
                }
                _ => {
                    eprintln!("Unknown field: {}", name);
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown field"));
                }
            }
        }

        let title = title.ok_or(Self::missing_field("title"))?;
        let description = description.ok_or(Self::missing_field("description"))?;
        let date = date.ok_or(Self::missing_field("date"))?.parse().unwrap();
        let remote = remote.ok_or(Self::missing_field("remote"))?.parse().unwrap();
        let event_url = event_url.ok_or(Self::missing_field("event_url"))?;
        let image = image.ok_or(Self::missing_field("image"))?.to_vec();
        let original_filename = original_filename.ok_or(Self::missing_field("image"))?;

        Ok(Self::new(title, description, date, remote, event_url, image, original_filename))
    }

    fn missing_field(name: &str) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Missing field: {}", name))
    }
}
