use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub remember: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}