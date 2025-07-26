use axum::{
    extract::{self, State},
    routing::post,
    Router,
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::{NotSet, Set}, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use crate::{routes::http_models::{
    FindPasswordForm, LoginRequest, Message, RegisterUserRequest, ResponseEntity
}};
use crate::database::prelude::*;
use crate::database::users;
use crate::AppState;

async fn register_user(State(state): State<AppState>, extract::Json(RegisterUserRequest { email, password }): extract::Json<RegisterUserRequest>) -> Json<ResponseEntity<users::Model>> {
    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(email),
        password: Set(password),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    let result = user.insert(&state.conn).await.unwrap();

    Json(
        ResponseEntity {
            code: 1,
            message: "User registered successfully".to_string(),
            response: Some(result),
        }
    )
}


async fn login(State(state): State<AppState>,  extract::Json(LoginRequest { email, password }): extract::Json<LoginRequest>) -> Json<ResponseEntity<users::Model>> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(&state.conn)
        .await
        .unwrap();

    if let Some(user) = user {
        if user.password == password {
            Json(ResponseEntity {
                code: 1,
                message: "Login successful".to_string(),
                response: Some(user),
            })
        } else {
            Json(ResponseEntity {
                code: 0,
                message: "Invalid password".to_string(),
                response: None,
            })
        }
    } else {
        Json(ResponseEntity {
            code: 0,
            message: "User not found".to_string(),
            response: None,
        })
    }
}

async fn find_password(extract::Form(FindPasswordForm { email}): extract::Form<FindPasswordForm>) -> Json<Message> {
    Json(Message { code: 1, message: format!("Success, we send an email to {email}") })
}

pub fn login_route() -> Router<AppState> { 
  Router::new()
  .route("/login", post(login))
  .route("/register", post(register_user))
  .route("/findpassword", post(find_password))
}

