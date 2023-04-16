use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::responses::UserResponse,
    db::get_connection_from_pool,
    models::UserCreateModel,
    services::{UserService, UserServiceError},
    AppState,
};

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct UserController;

impl UserController {
    pub async fn create(
        Extension(state): Extension<Arc<AppState>>,
        Json(user_request): Json<UserCreateRequest>,
    ) -> Result<UserResponse, UserControllerError> {
        let new_user = UserCreateModel {
            email: user_request.email,
            password: user_request.password,
        };

        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| UserControllerError::InternalError)?;

        let user = UserService::create_user(db_connection.as_mut(), new_user)
            .await
            .map_err(|err| match err {
                UserServiceError::AlreadyExistsError => UserControllerError::CreateConflict,
                _ => UserControllerError::InternalError,
            })?;

        let user_response = UserResponse {
            id: user.id,
            email: user.email,
        };

        Ok(user_response)
    }

    pub async fn read(
        Extension(state): Extension<Arc<AppState>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<UserResponse, UserControllerError> {
        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| UserControllerError::InternalError)?;

        let user = UserService::get_user_by_id(db_connection.as_mut(), &user_id)
            .await
            .map_err(|err| match err {
                UserServiceError::NotFoundError => UserControllerError::NotFound,
                _ => UserControllerError::InternalError,
            })?;

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        Ok(UserResponse {
            id: user.id,
            email: user.email,
        })
    }

    pub async fn update(
        Extension(_state): Extension<Arc<AppState>>,
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, format!("/users/{}", user_id))
    }

    pub async fn delete(
        Extension(_state): Extension<Arc<AppState>>,
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, format!("/users/{}", user_id))
    }
}

pub enum UserControllerError {
    InternalError,
    NotFound,
    CreateConflict,
}

impl UserControllerError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InternalError => "An error occurred while processing your request. Please try again later.",
            Self::NotFound => "The requested user was not found.",
            Self::CreateConflict => "An account is already associated with that email. Please login or use a different email.",
        }
    }
}

impl IntoResponse for UserControllerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
