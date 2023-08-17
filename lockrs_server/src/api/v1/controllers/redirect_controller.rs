use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use crate::{
    api::v1::responses::{RedirectListResponse, RedirectResponse},
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
        State(state): State<AppState>,
        Path(client_id): Path<String>,
    ) -> Result<RedirectListResponse, RedirectControllerError> {
        tracing::trace!(method = "read_all", client_id = client_id);

        let db_context = &state.db_context;
        let redirect_repository = &*state.repository_container.as_ref().redirect_repository;

        let redirects = RedirectService::get_redirects_from_client(
            db_context,
            redirect_repository,
            client_id.as_str(),
        )
        .await
        .map_err(RedirectControllerError::from)?;

        Ok(RedirectListResponse {
            redirects: redirects
                .into_iter()
                .map(|r| RedirectResponse {
                    id: r.id,
                    client_id: r.client_id,
                    uri: r.uri,
                })
                .collect::<Vec<RedirectResponse>>(),
        })
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
        State(state): State<AppState>,
        Path(redirect_id): Path<Uuid>,
    ) -> Result<RedirectResponse, RedirectControllerError> {
        tracing::trace!(method = "read", ?redirect_id);

        let db_context = &state.db_context;
        let redirect_repository = &*state.repository_container.as_ref().redirect_repository;

        let redirect =
            RedirectService::get_redirect_by_id(db_context, redirect_repository, &redirect_id)
                .await
                .map_err(RedirectControllerError::from)?;

        Ok(RedirectResponse {
            id: redirect.id,
            client_id: redirect.client_id,
            uri: redirect.uri,
        })
    }

    pub async fn delete(
        State(state): State<AppState>,
        Path(redirect_id): Path<Uuid>,
    ) -> Result<StatusCode, RedirectControllerError> {
        tracing::trace!(method = "delete", ?redirect_id);

        let db_context = &state.db_context;
        let redirect_repository = &*state.repository_container.as_ref().redirect_repository;

        RedirectService::delete_redirect_by_id(db_context, redirect_repository, &redirect_id)
            .await
            .map_err(RedirectControllerError::from)?;

        Ok(StatusCode::NO_CONTENT)
    }
}

pub enum RedirectControllerError {
    Unauthorized,
    InvalidClient,
    InvalidRedirect,
    AlreadyExists,
    NotFound,
    TooFewRedirects,
    NotDeleted,

    InternalError,
}

impl RedirectControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,

            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
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
            Self::TooFewRedirects => "Client must have at least one redirect uri assigned to it",
            Self::NotDeleted => "Unable to delete the redirect specified",

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
            RedirectServiceError::TooFewRedirects => Self::TooFewRedirects,
            RedirectServiceError::NotDeleted => Self::NotDeleted,

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
