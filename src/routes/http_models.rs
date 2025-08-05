use std::str;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ResponseEntity<T> {
  pub code: i32,
  pub success: bool,
  pub message: String,
  pub data: Option<T>,
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
  pub email: String,
  pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
  pub avatar: String,
  pub email: String,
  pub nickname: String,
  pub roles: Vec<String>,
  pub permissions: Vec<String>,
  pub accessToken: String,
  pub refreshToken: String,
  pub expres: DateTime<Utc>,
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
  pub userId: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct GetWordsRequest {
  pub userId: Uuid,
}
