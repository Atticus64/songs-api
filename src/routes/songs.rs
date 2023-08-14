use axum::{routing::get, Router};

use crate::controllers::songs::get_songs;

pub fn songs_routes() -> Router {
    Router::new().route("/", get(get_songs))
}
