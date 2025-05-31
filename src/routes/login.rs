use axum::{
    extract,
    routing::post,
    Router,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use crate::routes::http_models::{
    RegisterUserRequest,
    ResponseEntity,
    FindPasswordForm,
    Message,
};
use crate::database::prelude::*;
use crate::database::user;

async fn register_user(extract::Json(RegisterUserRequest { email, password }): extract::Json<RegisterUserRequest>) -> Json<ResponseEntity<user::Model>> {
    // payload is a `CreateUser`
    if email == "secret#test.com" {
        Json(
            ResponseEntity { 
                code: 1, 
                message: format!("Register user success! {email}"), 
                response: Some(user::Model {
                    id: Uuid::new_v4(), 
                    email: email, 
                    password: "masked".to_owned(), 
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),   
                })
            })
    } else {
        Json(ResponseEntity { code: 0, message: format!("Wrong email received {email}"), response: None})
    }
}

async fn find_password(extract::Form(FindPasswordForm { email}): extract::Form<FindPasswordForm>) -> Json<Message> {
    Json(Message { code: 1, message: format!("Success, we send an email to {email}") })
}

pub fn login_route() -> Router { 
  Router::new()
    .route("/login", post(register_user))
    .route("/findpassword", post(find_password))
}

