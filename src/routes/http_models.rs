use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub struct ResponseEntity<T> {
    pub code: i32,
    pub message: String,
    pub response: Option<T>
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
}