use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::responses::UserResponse,
    models::UserCreateModel,
    services::{UserService, UserServiceError},
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
        Json(user_request): Json<UserCreateRequest>,
    ) -> Result<Json<UserResponse>, UserControllerError> {
        let new_user = UserCreateModel {
            email: user_request.email,
            password: user_request.password,
        };

        let user = UserService::create_user(new_user).map_err(|err| match err {
            UserServiceError::AlreadyExistsError => UserControllerError::CreateConflict,
            _ => UserControllerError::InternalError,
        })?;

        let user_response = UserResponse {
            id: user.id,
            email: user.email,
        };

        Ok(Json(user_response))
    }

    pub async fn read(
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<UserResponse>, UserControllerError> {
        let user = UserService::get_user_by_id(&user_id).map_err(|err| match err {
            UserServiceError::NotFoundError => UserControllerError::NotFound,
            _ => UserControllerError::InternalError,
        })?;

        Ok(Json(UserResponse {
            id: user.id,
            email: user.email,
        }))
    }

    pub async fn update(Path(user_id): Path<Uuid>) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, format!("/users/{}", user_id))
    }

    pub async fn delete(Path(user_id): Path<Uuid>) -> impl IntoResponse {
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
