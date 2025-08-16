use std::str;

use chrono::prelude::*;
use rust_decimal::Decimal;
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

#[derive(Serialize)]
pub struct WordsResponse {
  pub words: Vec<String>,
  pub size: i32,
}

#[derive(Serialize)]
pub struct Summary {
  pub name: String,
  pub count: i32,
}

#[derive(Serialize)]
pub struct SummaryResponse {
  pub wordsSummary: Summary,
  pub finishedSummary: Summary,
}


#[derive(Deserialize)]
pub struct FindPasswordForm {
  pub email: String,
}

#[derive(Serialize)]
pub struct Message {
  pub message: String,
}

#[derive(Deserialize)]
pub struct CreateSheetRequest {
  pub userId: Uuid,
  pub name: String,
}

#[derive(Serialize)]
pub struct CreateSheetResponse {
  pub id: i32,
  pub name: String,
  pub score: f64,
  pub is_finished: bool,
}

#[derive(Serialize)]
pub struct SheetResponse {
  pub id: i32,
  pub name: String,
  pub score: Decimal,
  pub is_finished: bool,
}
#[derive(Serialize)]
pub struct GetSheetsResponse {
  pub sheets: Vec<SheetResponse>,
}

#[derive(Deserialize)]
pub struct AddWordsToSheetRequest {
  pub sheetId: i32,
  pub wordIds: Vec<i32>,
}

#[derive(Deserialize)]
pub struct CreateWrodsRequest {
  pub wordList: Vec<String>,
  pub userId: Uuid,
}

#[derive(Deserialize)]
pub struct GetWordsRequest {
  pub userId: Uuid,
}
