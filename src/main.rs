mod models;
mod handlers;
mod routes;
mod app;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    //Setting up router
    let app = create_app();

    //Setting up listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    //Starting axum server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(||async { "Hello, World!" }))
}