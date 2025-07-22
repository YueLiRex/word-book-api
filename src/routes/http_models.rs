use std::str;

use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
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

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Debug)]
pub struct FindPasswordForm {
    pub email: String,
}

#[derive(Serialize)]
pub struct Message {
    pub code: i32,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateWrodsRequest {
    pub wordList: Vec<String>,
}
