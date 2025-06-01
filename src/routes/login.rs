use axum::{
    extract,
    routing::post,
    Router,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use crate::routes::http_models::{
    FindPasswordForm, LoginRequest, Message, RegisterUserRequest, ResponseEntity
};
use crate::database::prelude::*;
use crate::database::users;

async fn find_password(extract::Form(FindPasswordForm { email}): extract::Form<FindPasswordForm>) -> Json<Message> {
    Json(Message { code: 1, message: format!("Success, we send an email to {email}") })
}

pub fn login_route() -> Router { 
  Router::new()
    .route("/findpassword", post(find_password))
}

