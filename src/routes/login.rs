use axum::{
    extract,
    routing::post,
    routing::get,
    Router,
    Json,
};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use crate::routes::http_models::RegisterUserRequest;
use crate::routes::http_models::ResponseEntity;

#[derive(Serialize)]
struct User {
    id: Uuid,
    username: String,
}

#[derive(Deserialize, Debug)]
struct FindPasswordForm {
    email: String,
}

#[derive(Serialize)]
struct Message {
    code: i32,
    message: String,
}

async fn register_user(extract::Json(RegisterUserRequest { email, password }): extract::Json<RegisterUserRequest>) -> Json<ResponseEntity<User>> {
    // payload is a `CreateUser`
    if email == "secret#test.com" {
        Json(ResponseEntity { code: 1, message: format!("Register user success! {email}"), response: Some(User { id: Uuid::new_v4(), username: email })})
    } else {
        Json(ResponseEntity { code: 1, message: format!("Register user success! {email}"), response: None})
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

