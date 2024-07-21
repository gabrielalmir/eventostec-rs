use sqlx::PgPool;
use super::models::Event;

pub struct EventRepository {
    pool: PgPool
}

impl EventRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_event(&self, event: Event) -> Result<Event, sqlx::Error> {
        let sql = r#"
            INSERT INTO event (id, title, description, image_url, event_url, date, remote)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#;
        let query = sqlx::query(sql)
            .bind(&event.id)
            .bind(&event.title)
            .bind(&event.description)
            .bind(&event.image_url)
            .bind(&event.event_url)
            .bind(&event.date)
            .bind(&event.remote)
            .execute(&self.pool)
            .await;

        match query {
            Ok(_) => Ok(event),
            Err(e) => {
                eprintln!("Failed to execute query: {:?}", e);
                Err(e)
            }
        }
    }
}
