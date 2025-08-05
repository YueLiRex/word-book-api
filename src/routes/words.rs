use std::{ops::Not, result, str::FromStr};

use axum::{
  extract::{self, State},
  routing::{delete, get, post, put},
  Json, Router,
};
use chrono::Utc;
use reqwest::header::MaxSizeReached;
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::{NotSet, Set},
  ColumnTrait, DatabaseConnection, DbConn, EntityTrait, QueryFilter, TryIntoModel,
};
use uuid::Uuid;

use crate::{database::prelude::*, routes::http_models::{Message, WordsResponse}};
use crate::{routes::http_models::GetWordsRequest, AppState};

use crate::database::words;

use super::http_models::{CreateWrodsRequest, ResponseEntity};

async fn add_words(
  State(state): State<AppState>,
  Json(CreateWrodsRequest { wordList, userId }): Json<CreateWrodsRequest>,
) -> Json<ResponseEntity<Message>> {
  let records: Vec<words::ActiveModel> = wordList
    .into_iter()
    .map(|word| words::ActiveModel {
      id: NotSet,
      word: Set(word),
      is_selected: Set(false),
      user_id: Set(userId),
      created_at: Set(Utc::now().naive_utc()),
      updated_at: Set(Utc::now().naive_utc()),
    })
    .collect();

  let result = Words::insert_many(records).exec(&state.conn).await;

  match result {
    Ok(_) => {
      Json(ResponseEntity {
        code: 1,
        success: true,
        message: "Words added successfully".to_string(),
        data: None,
      })
    }
    Err(e) => {
      eprintln!("Error adding words: {}", e);
      Json(ResponseEntity {
        code: 0,
        success: false,
        message: format!("Error: {:?}", e.to_string()),
        data: None,
      })
    }
  }
}

async fn get_words(
  State(state): State<AppState>,
  Json(GetWordsRequest { userId }): Json<GetWordsRequest>,
) -> Json<ResponseEntity<WordsResponse>> {
  let records = Words::find()
    .filter(words::Column::UserId.eq(userId))
    .all(&state.conn)
    .await
    .unwrap();

  let words =records.iter().map(|record| record.word.clone()).collect::<Vec<String>>();
  let size = words.len() as i32;

  Json(ResponseEntity {
    code: 1,
    success: true,
    message: format!("Found {} words.", size),
    data: Some(WordsResponse { words, size }),
  })
}

pub fn words_route() -> Router<AppState> {
  Router::new()
  .route("/words", get(get_words).post(add_words))
}
