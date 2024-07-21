mod db;
mod event;
mod aws;

use std::env;
use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("Failed to load .env file");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let event_routes = event::handlers::routes().await;

    let app = Router::new()
        .nest("/api/event", event_routes);

    println!("> Server running on: http://{}", addr);
    let server = TcpListener::bind(&addr).await?;
    axum::serve(server, app).await?;

    Ok(())
}
