use axum::{
    extract,
    routing::post,
    Router,
    Json,
};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use crate::routes::http_models::{
    FindPasswordForm, LoginRequest, Message, RegisterUserRequest, ResponseEntity
};
use crate::database::prelude::*;
use crate::database::users;

async fn find_password(extract::Form(FindPasswordForm { email}): extract::Form<FindPasswordForm>) -> Json<Message> {
    Json(Message { code: 1, message: format!("Success, we send an email to {email}") })
}

async fn login(extract::Json(LoginRequest { email, password }): extract::Json<LoginRequest>) -> Json<ResponseEntity<users::Model>> {

   Json(ResponseEntity { 
            code: 1, 
            message: "Login successful".to_string(),
            response: Some(users::Model {
                id: Uuid::new_v4(),
                email: email.to_string(),
                password: password.to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            })
    })
}

pub fn login_route() -> Router { 
  Router::new()
  .route("/login", post(login))
  .route("/findpassword", post(find_password))
}

