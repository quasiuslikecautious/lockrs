use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use url::Url;

use crate::{
    api::v1::responses::RedirectResponse,
    models::RedirectCreateModel,
    services::{ClientAuthService, ClientAuthServiceError, RedirectService, RedirectServiceError},
    utils::extractors::SessionJwt,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct RedirectCreateRequest {
    pub client_id: String,
    pub uri: Url,
}

pub struct RedirectController;

impl RedirectController {
    pub async fn read_all(
        State(_state): State<AppState>,
        Path(client_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::trace!(method = "read_all", client_id = client_id);

        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/clients/{}/redirects", client_id),
        )
    }

    pub async fn create(
        State(state): State<AppState>,
        SessionJwt(auth_info): SessionJwt,
        Json(new_redirect_request): Json<RedirectCreateRequest>,
    ) -> Result<RedirectResponse, RedirectControllerError> {
        tracing::trace!(
            method = "create",
            params = ?new_redirect_request
        );

        let new_redirect = RedirectCreateModel::new(
            new_redirect_request.client_id.as_str(),
            &new_redirect_request.uri,
        );

        let db_context = &state.db_context;
        let client_repository = &*state.repository_container.as_ref().client_repository;

        // validate client first
        ClientAuthService::verify_user(
            db_context,
            client_repository,
            new_redirect.client_id.as_str(),
            &auth_info.user_id,
        )
        .await
        .map_err(RedirectControllerError::from)?;

        let redirect_repository = &*state.repository_container.as_ref().redirect_repository;

        let redirect =
            RedirectService::create_redirect(db_context, redirect_repository, &new_redirect)
                .await
                .map_err(RedirectControllerError::from)?;

        Ok(RedirectResponse {
            id: redirect.id,
            client_id: redirect.client_id,
            uri: redirect.uri,
        })
    }

    pub async fn read(
        State(_state): State<AppState>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::trace!(method = "read", redirect_id = redirect_id);

        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }

    pub async fn update(
        State(_state): State<AppState>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::trace!(method = "update", redirect_id = redirect_id);

        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }

    pub async fn delete(
        State(_state): State<AppState>,
        Path(redirect_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::trace!(method = "delete", redirect_id = redirect_id);

        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/redirects/{}", redirect_id),
        )
    }
}

pub enum RedirectControllerError {
    Unauthorized,
    InvalidClient,
    InvalidRedirect,
    AlreadyExists,
    NotFound,

    InternalError,
}

impl RedirectControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InvalidClient => StatusCode::BAD_REQUEST,
            Self::InvalidRedirect => StatusCode::BAD_REQUEST,
            Self::AlreadyExists => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,

            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::Unauthorized => {
                "You are missing the sufficient privledges to perform the requested operation."
            }
            Self::InvalidClient => "The provided client id is invalid.",
            Self::InvalidRedirect => "The provided redirect is invalid.",
            Self::AlreadyExists => {
                "The uri provided is already registered as a callback for the client specified"
            }
            Self::NotFound => "Unable to find a redirect matching the requested criteria.",

            Self::InternalError => {
                "An error has occurred while processing your request. Please try again later."
            }
        }
    }
}

impl From<RedirectServiceError> for RedirectControllerError {
    fn from(err: RedirectServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            RedirectServiceError::AlreadyExists => Self::AlreadyExists,
            RedirectServiceError::NotFound => Self::NotFound,
            RedirectServiceError::NotCreated => Self::InvalidRedirect,

            RedirectServiceError::InternalError => Self::InternalError,
        }
    }
}

impl From<ClientAuthServiceError> for RedirectControllerError {
    fn from(err: ClientAuthServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ClientAuthServiceError::NotFound => Self::InvalidClient,
            ClientAuthServiceError::InvalidUser => Self::Unauthorized,

            _ => Self::InternalError,
        }
    }
}

impl IntoResponse for RedirectControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
