use std::{ops::Not, result, str::FromStr};

use axum::{
    extract::{self, State}, routing::{delete, get, post, put}, Json, Router
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::{NotSet, Set}, ColumnTrait, DatabaseConnection, DbConn, EntityTrait, QueryFilter, TryIntoModel};
use uuid::Uuid;

use crate::{routes::http_models::GetWordsRequest, AppState};
use crate::database::prelude::*;

use crate::database::words;

use super::http_models::{CreateWrodsRequest, ResponseEntity};

async fn add_words(State(state): State<AppState>, Json(CreateWrodsRequest { wordList, userId }): Json<CreateWrodsRequest>) -> Json<ResponseEntity<words::Model>> {

  let records: Vec<words::ActiveModel> = wordList.into_iter().map(|word| {
    words::ActiveModel {
      id: NotSet,
      word: Set(word),
      is_selected: Set(false),
      user_id: Set(userId),
      created_at: Set(Utc::now().naive_utc()),
      updated_at: Set(Utc::now().naive_utc()),
    }
  }).collect();

  let result = words::Entity::insert_many(records)
    .exec(&state.conn)
    .await.unwrap();

  Json(
    ResponseEntity {
      code: 1,
      success: true,
      message: format!("Saved {:?} words.", result),
      data: None,
    }
  )
}

async fn get_words(State(state): State<AppState>, Json(GetWordsRequest { userId }): Json<GetWordsRequest>) -> Json<ResponseEntity<Vec<words::Model>>> {
  let records = Words::find()
    .filter(words::Column::UserId.eq(userId))
    .all(&state.conn)
    .await.unwrap();

  Json(
    ResponseEntity {
      code: 1,
      success: true,
      message: format!("Found {} words.", records.len()),
      data: Some(records),
    }
  )
}

pub fn words_route() -> Router<AppState> {
  Router::new()
    .route("/words", get(get_words).post(add_words))
}
