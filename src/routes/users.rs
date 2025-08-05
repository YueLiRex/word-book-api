use crate::database::prelude::*;
use crate::database::users;
use crate::database::profiles;
use crate::routes::http_models::LoginResponse;
use crate::routes::http_models::{
  FindPasswordForm, LoginRequest, Message, RegisterUserRequest, ResponseEntity,
};
use crate::AppState;
use axum::{
  extract::{self, State},
  routing::post,
  Json, Router,
};
use chrono::Utc;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait, EntityTrait, QueryFilter,
};
use uuid::Uuid;

async fn register_user(
  State(state): State<AppState>,
  extract::Json(RegisterUserRequest { email, password }): extract::Json<RegisterUserRequest>,
) -> Json<ResponseEntity<users::Model>> {
  let userId = Uuid::new_v4();
  let user = users::ActiveModel {
    id: Set(userId),
    email: Set(email),
    password: Set(password),
    created_at: Set(Utc::now().naive_utc()),
    updated_at: Set(Utc::now().naive_utc()),
  };
  let profile = profiles::ActiveModel {
    id: NotSet,
    user_id: Set(userId),
    username: Set("Default Username".to_string()),
    nickname: Set("Default Nickname".to_string()),
    avatar: Set("default_avatar.png".to_string()),
    roles: Set("user".to_string()),
    permissions: Set("read,write".to_string()),
    created_at: Set(Utc::now().naive_utc()),
    updated_at: Set(Utc::now().naive_utc()),
  };

  let userResult = user.insert(&state.conn).await.unwrap();
  let profileResult = profile.insert(&state.conn).await.unwrap();

  Json(ResponseEntity {
    code: 1,
    success: true,
    message: "User registered successfully".to_string(),
    data: Some(userResult),
  })
}

async fn login(
  State(state): State<AppState>,
  extract::Json(LoginRequest { email, password }): extract::Json<LoginRequest>,
) -> Json<ResponseEntity<LoginResponse>> {
  let userWithProfile = Users::find().find_also_related(Profiles)
    .filter(users::Column::Email.eq(email))
    .one(&state.conn)
    .await
    .unwrap();

  match userWithProfile {
    Some((user, Some(profile))) if user.password == password => {
      let login_response = LoginResponse {
        avatar: profile.avatar,
        email: user.email.clone(),
        nickname: profile.nickname,
        roles: profile.roles.split(",").map(|s| s.to_string()).collect(),
        permissions: profile.permissions.split(",").map(|s| s.to_string()).collect(),
        accessToken: "dummy_access_token".to_string(),
        refreshToken: "dummy_refresh_token".to_string(),
        expres: Utc::now() + chrono::Duration::days(7),
      };
      Json(ResponseEntity {
        code: 1,
        success: true,
        message: "Login successful".to_string(),
        data: Some(login_response),
      })
    }
    _ => Json(ResponseEntity {
      code: 0,
      success: false,
      message: "Invalid email or password".to_string(),
      data: None,
    }),
      
  }
}

async fn find_password(
  extract::Form(FindPasswordForm { email }): extract::Form<FindPasswordForm>,
) -> Json<Message> {
  Json(Message {
    code: 1,
    message: format!("Success, we send an email to {email}"),
  })
}

pub fn login_route() -> Router<AppState> {
  Router::new()
    .route("/login", post(login))
    .route("/register", post(register_user))
    .route("/findpassword", post(find_password))
}
