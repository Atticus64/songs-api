use std::net::SocketAddr;

use axum::{routing::get, Router};

use crate::{controllers::hello_world, routes::songs::songs_routes, db::test_connection};
use http::{header, Method};
use tower_http::cors::CorsLayer;

mod controllers;
mod models;
mod routes;
mod db;
mod schema;

#[tokio::main]
async fn main() {

    let port = dotenvy::var("PORT").unwrap_or("8080".to_string());
    test_connection().unwrap();

    let origins = [
        "http://localhost:8080".parse().unwrap(),
        "http://localhost:3000".parse().unwrap(),
        "https://search-alabanzas.vercel.app".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_headers(vec![
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
        ])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
            Method::CONNECT,
            Method::PATCH,
            Method::TRACE,
        ])
        .allow_origin(origins);

    let app = Router::new()
        .nest("/songs", songs_routes())
        .route("/", get(hello_world))
        .layer(cors);

    println!("Listening http://localhost:{}", port);
    let address = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
