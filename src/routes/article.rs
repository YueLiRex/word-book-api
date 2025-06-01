// use std::ops::Not;

// use axum::{
//     extract::{self, State}, routing::{delete, get, post, put}, Json, Router
// };
// use chrono::Utc;
// use sea_orm::{ActiveModelTrait, ActiveValue::{NotSet, Set}, DatabaseConnection, DbConn, TryIntoModel};
// use uuid::Uuid;


// use crate::{database::prelude::Article, AppState};

// use crate::database::article;

// use super::http_models::{CreateArticleRequest, ResponseEntity};

// async fn new_article(State(state): State<AppState>, Json(CreateArticleRequest { title}): Json<CreateArticleRequest>) -> Json<ResponseEntity<article::Model>> {
  
//   let article = article::ActiveModel {
//     id: Set(Uuid::new_v4()),
//     title: Set(title.clone()),
//     is_draft: Set(true),
//     content: Set("".to_owned()),
//     created_at: Set(Utc::now().naive_utc()),
//     updated_at: Set(Utc::now().naive_utc()),
//   };

//   let dbResult = article.insert(&state.conn).await.unwrap();
//   let savedArticle = dbResult.try_into_model().unwrap();

//   Json(
//     ResponseEntity {
//       code: 1,
//       message: format!("New article created! {title}"),
//       response: Some(savedArticle),
//     }
//   )
// }

// // async fn get_article() -> Json<ResponseEntity<ArticleContent>> {

// // }

// // async fn update_article() -> Json<ResponseEntity<ArticleContent>> {

// // }

// // async fn delete_article() -> Json<ResponseEntity<Message>> {

// // }

// pub fn article_route() -> Router<AppState> {
//   Router::new()
//     .route("/article", post(new_article))
//     // .route("/article", get(get_article))
//     // .route("/article", put(update_article))
//     // .route("/article", delete(delete_article))
// }
