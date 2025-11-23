use axum::{routing::get, Router};
use axum::body::Body;
use http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn index_returns_hello() {
    let app = Router::new()
    .route("/", get(|| async { "Hello,Axum!" }));

    let response = app.oneshot(
        Request::builder()
            .uri("/")
            .body(Body::empty())
        .unwrap()
    ).await
     .unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}