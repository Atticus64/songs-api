use std::net::SocketAddr;

use axum::{routing::get, Router};

use crate::{controllers::hello_world, routes::songs::songs_routes};
use tower_http::cors::{Any, CorsLayer};
use http::Method;

mod controllers;
mod models;
mod routes;
mod schema;

#[tokio::main]
async fn main() {
    let port = dotenvy::var("PORT").unwrap_or("8080".to_string());

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);


    let app = Router::new()
        .layer(cors)
        .nest("/songs", songs_routes())
        .route("/", get(hello_world));

    println!("Listening http://localhost:{}", port);
    let address = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
