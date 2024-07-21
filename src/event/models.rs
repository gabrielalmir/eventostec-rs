use serde::Serialize;

#[derive(Serialize)]
pub struct Event {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub event_url: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub remote: bool,
}

impl Event {
    pub fn new(title: String, description: String, image_url: String, event_url: String, date: i64, remote: bool) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            description,
            image_url,
            event_url,
            date: Self::to_timestamp(date),
            remote,
        }
    }

    fn to_timestamp(date: i64) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::<chrono::Utc>::from_timestamp_millis(date).unwrap()
    }
}
