use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::v1::responses::UserResponse,
    models::UserUpdateModel,
    services::{UserService, UserServiceError},
    utils::extractors::SessionJwt,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct UserUpdateRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct UserController;

impl UserController {
    pub async fn read(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<UserResponse, UserControllerError> {
        tracing::trace!(method = "read", id = user_id.to_string(),);

        let db_context = &state.db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;
        let user = UserService::get_user_by_id(db_context, user_repository, &user_id)
            .await
            .map_err(|err| {
                tracing::error!(error = %err);

                match err {
                    UserServiceError::NotFound => UserControllerError::NotFound,
                    _ => UserControllerError::Internal,
                }
            })?;

        Ok(UserResponse {
            id: user.id,
            email: user.email,
        })
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
        Json(update_user_request): Json<UserUpdateRequest>,
    ) -> Result<UserResponse, UserControllerError> {
        tracing::trace!(
            method = "update",
            id = user_id.to_string(),
            data = ?update_user_request,
        );

        let update_user = UserUpdateModel {
            email: update_user_request.email,
        };

        let db_context = &state.db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;
        let user =
            UserService::update_user_by_id(db_context, user_repository, &user_id, &update_user)
                .await
                .map_err(UserControllerError::from)?;

        Ok(UserResponse {
            id: user.id,
            email: user.email,
        })
    }

    pub async fn delete(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<StatusCode, UserControllerError> {
        tracing::trace!(method = "delete", id = user_id.to_string());

        let db_context = &state.db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;
        UserService::delete_user_by_id(db_context, user_repository, &user_id)
            .await
            .map_err(|err| {
                tracing::error!(error = %err);

                match err {
                    UserServiceError::NotFound => UserControllerError::NotFound,
                    _ => UserControllerError::Internal,
                }
            })?;

        Ok(StatusCode::NO_CONTENT)
    }
}

pub enum UserControllerError {
    Jwt,
    NotFound,
    AlreadyExists,
    BadRequest,
    Internal,
}

impl UserControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::Jwt => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::AlreadyExists => StatusCode::BAD_REQUEST,

            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::Jwt => "You do not have permission to view the requested resource.",
            Self::AlreadyExists => "An account is already associated with that email. Please login or use a different email.",
            Self::NotFound => "The requested user was not found.",

            Self::BadRequest => "Unable to perform the operation as requested.",
            Self::Internal => "An error occurred while processing your request. Please try again later.",
        }
    }
}

impl From<UserServiceError> for UserControllerError {
    fn from(err: UserServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            UserServiceError::AlreadyExists => Self::AlreadyExists,
            UserServiceError::NotFound => Self::NotFound,

            UserServiceError::InternalError => Self::Internal,

            _ => Self::BadRequest,
        }
    }
}

impl IntoResponse for UserControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
