use std::{ops::Not, result, str::FromStr};

use axum::{
  extract::{self, State},
  routing::{delete, get, post, put},
  Json, Router,
};
use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ActiveValue::{NotSet, Set}, ColumnTrait, DatabaseConnection, DbConn, EntityTrait, PaginatorTrait, QueryFilter, TryIntoModel
};
use uuid::Uuid;

use crate::{database::prelude::*, routes::http_models::{ Message, SummaryResponse, WordsResponse }};
use crate::{routes::http_models::GetWordsRequest, AppState};

use crate::database::words;

use super::http_models::{CreateWrodsRequest, ResponseEntity, Summary};

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
      is_finished: NotSet,
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

async fn get_words_summary(
  State(state): State<AppState>,
  extract::Json(GetWordsRequest { userId }): extract::Json<GetWordsRequest>,
) -> Json<ResponseEntity<SummaryResponse>> {
  let uuid = "f4c84bef-2280-4c4e-ba1f-aa44446feed7".to_string();
  let count = Words::find()
    .filter(words::Column::UserId.eq(uuid))
    .count(&state.conn)
    .await
    .unwrap_or(0);

  let finishedCount = Words::find()
    .filter(words::Column::UserId.eq(userId))
    .filter(words::Column::IsFinished.eq(true))
    .count(&state.conn)
    .await
    .unwrap_or(0); 


  let wordsSummary = Summary {
    name: "Total Words".to_string(),
    count: count as i32,
  };

  let finishedSummary = Summary { 
    name: "Finished Words".to_string(),
    count: finishedCount as i32,
  };
  // let finishedSummary = Summary { name: "Finished Words", count: () }

  Json(ResponseEntity {
    code: 1,
    success: true,
    message: format!("Total words count: {}", count),
    data: Some(SummaryResponse {
      wordsSummary,
      finishedSummary,
    })
  })
}

pub fn words_route() -> Router<AppState> {
  Router::new()
  .route("/words", get(get_words).post(add_words))
  .route("/words/summary", post(get_words_summary))
}
