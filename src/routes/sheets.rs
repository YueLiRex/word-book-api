use axum::{
  extract::{self, State},
  routing::{delete, get, post, put},
  Json, Router,
};
use chrono::Utc;
use sea_orm::{
  prelude::*, ActiveValue::{NotSet, Set}, ColumnTrait, EntityTrait, QueryFilter
};

use crate::{database::prelude::*, routes::http_models::{ AddWordsToSheetRequest, CreateSheetRequest, CreateSheetResponse, GetSheetsResponse, Message, SheetResponse }};
use crate::{routes::http_models::GetWordsRequest, AppState};

use crate::database::{sheets, sheets_words};

use super::http_models::{ResponseEntity};

async fn get_sheets(
  State(state): State<AppState>,
  Json(GetWordsRequest { userId }): Json<GetWordsRequest>,
) -> Json<ResponseEntity<GetSheetsResponse>> {
  let records = Sheets::find()
    .filter(sheets::Column::UserId.eq(userId))
    .all(&state.conn)
    .await
    .unwrap();

  let sheets: Vec<SheetResponse> = records
    .into_iter()
    .map(|sheet| SheetResponse {
      id: sheet.id,
      name: sheet.name,
      score: sheet.score,
      is_finished: sheet.is_finished,
    })
    .collect();

  Json(ResponseEntity {
    code: 1,
    success: true,
    message: "Sheets retrieved successfully".to_string(),
    data: Some(GetSheetsResponse { sheets }),
  })
}

async fn create_sheet(
  State(state): State<AppState>,
  Json(CreateSheetRequest { userId, name }): Json<CreateSheetRequest>,
) -> Json<ResponseEntity<Message>> {  
  let sheet = sheets::ActiveModel {
    id: NotSet,
    name: Set(name),
    score: Set(Decimal::ZERO),
    is_finished: Set(false),
    created_at: Set(Utc::now().naive_utc()),
    updated_at: Set(Utc::now().naive_utc()),
    user_id: Set(userId),
  };

  let result = Sheets::insert(sheet).exec(&state.conn).await;

  match result {
    Ok(_) => {
      Json(ResponseEntity {
        code: 1,
        success: true,
        message: "Sheet created successfully".to_string(),
        data: None,
      })
    }
    Err(e) => {
      eprintln!("Error creating sheet: {}", e);
      Json(ResponseEntity {
        code: 0,
        success: false,
        message: format!("Error: {:?}", e.to_string()),
        data: None,
      })
    }
  }
} 

async fn add_words_to_sheet(
  State(state): State<AppState>,
  Json(AddWordsToSheetRequest { sheetId, wordIds }): Json<AddWordsToSheetRequest>,
) -> Json<ResponseEntity<Message>> {

  let active_records = wordIds.iter().map( |word_id| {
    sheets_words::ActiveModel {
      id: NotSet,
      sheet_id: Set(sheetId),
      word_id: Set(*word_id),
    }
  }).collect::<Vec<_>>();

  let result = SheetsWords::insert_many(active_records)
    .exec(&state.conn)
    .await;

  match result {
    Ok(_) => {
      Json(ResponseEntity {
        code: 1,
        success: true,
        message: "Words added to sheet successfully".to_string(),
        data: None,
      })
    }
    Err(e) => {
      eprintln!("Error adding words to sheet: {}", e);
      Json(ResponseEntity {
        code: 0,
        success: false,
        message: format!("Error: {:?}", e.to_string()),
        data: None,
      })
    }
  }
}

pub fn sheets_route() -> Router<AppState> {
  Router::new()
  .route("/sheet", post(get_sheets))
  .route("/sheet/create", post(create_sheet))
  .route("/sheet/add-words",post(add_words_to_sheet))
}
