use axum::{Router, routing::get, response::Json, extract::Path};
use axum::extract::Query;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Login {
    username: String,
    password: String
}
impl Login {
    fn welcome(&self) -> String {
        format!("Welcome, {}", self.username)
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root_handle))
        .route("/users/{user_id}", get(get_user_id))
        .route("/login", get(login_handle))

}

//Return a static string as response
async fn root_handle() -> Json<String> {
    Json("Home Page".to_string())
}

async fn login_handle(Query(user): Query<Login>) -> Result<Json<String>, StatusCode> {
    if user.username == "admin" && user.password == "admin" {
        return Ok(Json(user.welcome()));
    }
    Err(StatusCode::UNAUTHORIZED)
}

async fn get_user_id(Path(user_id): Path<u64>) -> String {
    format!("Fetching user profile for ID: {}", user_id)
}