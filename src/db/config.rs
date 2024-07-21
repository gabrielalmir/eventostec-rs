use sqlx::Pool;

pub async fn pool() -> Pool<sqlx::Postgres> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::connect(&db_url).await.expect("Failed to create pool")
}
