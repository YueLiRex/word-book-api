use axum::{
    extract, routing::{delete, get, post, put}, Json, Router
};
use uuid::Uuid;


use crate::database::prelude::*;

use crate::database::article;

use super::http_models::{CreateArticleRequest, ResponseEntity};

async fn new_article(extract::Json(CreateArticleRequest { title}): extract::Json<CreateArticleRequest>) -> Json<ResponseEntity<article::Model>> {
  
  Json(
    ResponseEntity {
      code: 1,
      message: format!("New article created! {title}"),
      response: Some(article::Model {
        id: Uuid::new_v4(),
        title: "test_title".to_owned(),
        is_draft: true,
        content: "test_content".to_owned(),
        created_at: chrono::offset::Utc::now().naive_utc(),
        updated_at: chrono::offset::Utc::now().naive_utc(),
      }),
    }
  )
}

// async fn get_article() -> Json<ResponseEntity<ArticleContent>> {

// }

// async fn update_article() -> Json<ResponseEntity<ArticleContent>> {

// }

// async fn delete_article() -> Json<ResponseEntity<Message>> {

// }

pub fn article_route() -> Router {
  Router::new()
    .route("/article", post(new_article))
    // .route("/article", get(get_article))
    // .route("/article", put(update_article))
    // .route("/article", delete(delete_article))
}
