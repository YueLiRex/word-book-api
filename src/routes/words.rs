use std::{ops::Not, result};

use axum::{
    extract::{self, State}, routing::{delete, get, post, put}, Json, Router
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::{NotSet, Set}, DatabaseConnection, DbConn, EntityTrait, TryIntoModel};
use uuid::Uuid;

use crate::AppState;
use crate::database::prelude::*;

use crate::database::words;

use super::http_models::{CreateWrodsRequest, ResponseEntity};

async fn add_words(State(state): State<AppState>, Json(CreateWrodsRequest { wordList }): Json<CreateWrodsRequest>) -> Json<ResponseEntity<words::Model>> {

  let records: Vec<words::ActiveModel> = wordList.into_iter().map(|word| {
    words::ActiveModel {
      id: NotSet,
      word: Set(word),
      is_selected: Set(false),
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
      message: format!("Saved {:?} words.", result),
      response: None,
    }
  )
}

async fn get_words(State(state): State<AppState>) -> Json<ResponseEntity<Vec<words::Model>>> {
  let records = words::Entity::find().all(&state.conn).await.unwrap();
  Json(
    ResponseEntity {
      code: 1,
      message: format!("Found {} words.", records.len()),
      response: Some(records),
    }
  )
}




pub fn words_route() -> Router<AppState> {
  Router::new()
    .route("/words", get(get_words).post(add_words))
}
