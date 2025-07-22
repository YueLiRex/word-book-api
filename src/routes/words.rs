use std::ops::Not;

use axum::{
    extract::{self, State}, routing::{delete, get, post, put}, Json, Router
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::{NotSet, Set}, DatabaseConnection, DbConn, TryIntoModel};
use uuid::Uuid;


use crate::database::prelude::*;

use crate::database::words;

use super::http_models::{CreateWrodsRequest, ResponseEntity};

async fn add_word(Json(CreateWrodsRequest { word}): Json<CreateWrodsRequest>) -> Json<ResponseEntity<words::Model>> {

  Json(
    ResponseEntity {
      code: 1,
      message: format!("Saved {:?} words.", word.len()),
      response: Some(words::Model { id: 1, word: "test".to_string(), is_selected: false, created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() }),
    }
  )
}


pub fn words_route() -> Router {
  Router::new()
    .route("/words", post(add_word))
}
