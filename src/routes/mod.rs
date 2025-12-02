use axum::{Router, extract::Path, http::StatusCode, routing::{get,post}};
use axum::extract::Query;
use crate::models::{Login, CreateUserPayload};
use axum::Json;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root_handle))
        .route("/users/{user_id}", get(get_user_id))
        .route("/users", post(create_user_handler))
        .route("/login", get(login_handle))

}

//Return a static string as response
async fn root_handle() -> String {
    "Home Page".to_string()
}

async fn login_handle(Query(data): Query<Login>) -> String {
    let remember = data.remember.unwrap_or(false);
    if data.username == "admin" && data.password == "admin" {
        return format!("Welcome {}, your session is set {}, it will not be remembered.", data.username, remember);
    }
    format!("Welcome guest!")
}

async fn get_user_id(Path(user_id): Path<u64>) -> String {
    format!("Fetching user profile for ID: {}", user_id)
}

async fn create_user_handler(Json(payload): Json<CreateUserPayload>) 
    -> (StatusCode, Json<CreateUserPayload>) {
        println!("Received user creation request: {:?}", payload);
        (StatusCode::CREATED, Json(payload))
}