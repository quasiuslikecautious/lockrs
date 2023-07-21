use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::{
        models::RegisterModel,
        responses::UserResponse,
        services::{AuthService, AuthServiceError},
    },
    models::UserUpdateModel,
    services::{UserService, UserServiceError},
    utils::extractors::SessionJwt,
    AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
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
        State(state): State<Arc<AppState>>,
        Json(register_request): Json<RegisterRequest>,
    ) -> Result<UserResponse, UserControllerError> {
        let registration = RegisterModel {
            email: register_request.email,
            password: register_request.password,
        };

        let db_context = &state.as_ref().db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;

        let user = AuthService::register_user(db_context, user_repository, &registration)
            .await
            .map_err(UserControllerError::from)?;

        let user_response = UserResponse {
            id: user.id,
            email: user.email,
        };

        Ok(user_response)
    }

    pub async fn read(
        State(state): State<Arc<AppState>>,
        SessionJwt(jwt): SessionJwt,
        Path(user_id): Path<Uuid>,
    ) -> Result<UserResponse, UserControllerError> {
        if jwt.user_id != user_id {
            return Err(UserControllerError::Jwt);
        }

        let db_context = &state.as_ref().db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;
        let user = UserService::get_user_by_id(db_context, user_repository, &user_id)
            .await
            .map_err(|err| {
                error!("USER CONTROLLER ERROR :: Read :: {}", err);
                match err {
                    UserServiceError::NotFound(_) => UserControllerError::NotFound,
                    _ => UserControllerError::Internal,
                }
            })?;

        Ok(UserResponse {
            id: user.id,
            email: user.email,
        })
    }

    pub async fn update(
        State(state): State<Arc<AppState>>,
        SessionJwt(jwt): SessionJwt,
        Path(user_id): Path<Uuid>,
        Json(update_user_request): Json<UserUpdateRequest>,
    ) -> Result<UserResponse, UserControllerError> {
        if jwt.user_id != user_id {
            return Err(UserControllerError::Jwt);
        }

        let update_user = UserUpdateModel {
            email: update_user_request.email,
        };

        let db_context = &state.as_ref().db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;
        let user =
            UserService::update_user_by_id(db_context, user_repository, &user_id, &update_user)
                .await
                .map_err(|err| {
                    error!("USER CONTROLLER ERROR :: Update :: {}", err);
                    match err {
                        UserServiceError::NotFound(_) => UserControllerError::NotFound,
                        _ => UserControllerError::Internal,
                    }
                })?;

        Ok(UserResponse {
            id: user.id,
            email: user.email,
        })
    }

    pub async fn delete(
        State(state): State<Arc<AppState>>,
        SessionJwt(jwt): SessionJwt,
        Path(user_id): Path<Uuid>,
    ) -> Result<StatusCode, UserControllerError> {
        if jwt.user_id != user_id {
            return Err(UserControllerError::Jwt);
        }

        let db_context = &state.as_ref().db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;
        UserService::delete_user_by_id(db_context, user_repository, &user_id)
            .await
            .map_err(|err| {
                error!("USER CONTROLLER ERROR :: Update :: {}", err);
                match err {
                    UserServiceError::NotFound(_) => UserControllerError::NotFound,
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

impl From<AuthServiceError> for UserControllerError {
    fn from(err: AuthServiceError) -> Self {
        error!("USER CONTROLLER ERROR :: {}", err);
        match err {
            AuthServiceError::AlreadyExists(_) => Self::AlreadyExists,
            _ => Self::Internal,
        }
    }
}

impl From<UserServiceError> for UserControllerError {
    fn from(err: UserServiceError) -> Self {
        error!("USER CONTROLLER ERROR :: {}", err);
        match err {
            UserServiceError::AlreadyExists(_) => Self::AlreadyExists, 
            UserServiceError::NotFound(_) => Self::NotFound,

            UserServiceError::NotCreated(_) => Self::BadRequest,
            UserServiceError::NotUpdated(_) => Self::BadRequest,
            UserServiceError::NotDeleted(_) => Self::BadRequest,

            UserServiceError::InternalError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for UserControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
