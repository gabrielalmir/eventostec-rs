use std::env;

use aws_sdk_s3::primitives::ByteStream;
use axum::extract::{self, Multipart};
use axum::response::IntoResponse;
use axum::Json;
use axum::{http, routing::post};
use sqlx::PgPool;

use crate::aws::config::AwsConfig;
use crate::db::config;

use super::dtos::CreateEventDTO;
use super::models::Event;
use super::repositories::EventRepository;

pub async fn create_event (
    extract::State(pool): extract::State<PgPool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, http::StatusCode> {
    let create_event_dto = CreateEventDTO::from_multipart(&mut multipart).await.unwrap();

    let event = Event::new(
        create_event_dto.title,
        create_event_dto.description,
        upload_image(&create_event_dto.image, &create_event_dto.original_filename).await.unwrap(),
        create_event_dto.event_url,
        create_event_dto.date,
        create_event_dto.remote,
    );

    let repository = EventRepository::new(pool);
    match repository.create_event(event).await {
        Ok(event) => Ok(Json(event).into_response()),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn upload_image (data: &[u8], original_filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let s3_client = AwsConfig::s3().await;
    let filename = format!("{}-{}", uuid::Uuid::new_v4(), original_filename);
    let key = filename.as_str();
    let body = ByteStream::from(data.to_vec());

    let bucket = env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME is not set");

    let response = s3_client.put_object()
        .bucket(&bucket)
        .key(key)
        .body(body)
        .send()
        .await;

    let url = AwsConfig::s3_url(&bucket, key).await;

    match response {
        Ok(_) => Ok(url),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn routes() -> axum::Router<()> {
    let pool = config::pool().await;

    axum::Router::new()
        .route("/", post(create_event))
        .with_state(pool)
}
