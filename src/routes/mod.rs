use axum::{Router, routing::get};


pub fn routes() -> Router {
    Router::new()
        .route("/", get(root_handle))
}

//Return a static string as response
async fn root_handle() -> &'static str {
    "Hello, Axum"
}