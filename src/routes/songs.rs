use axum::{routing::get, Router};

use crate::controllers::songs::{get_songs, search_song};

pub fn songs_routes() -> Router {
    Router::new()
        .route("/", get(get_songs))
        .route("/search", get(search_song))
}
