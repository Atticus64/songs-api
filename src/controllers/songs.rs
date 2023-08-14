use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    content: String,
}

pub async fn get_songs() -> Json<Message> {
    Json(Message {
        content: "hello songs 🎵".to_string(),
    })
}
