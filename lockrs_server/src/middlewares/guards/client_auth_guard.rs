use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};

use crate::{
    api::v1::services::SessionService, services::ClientService, utils::extractors::SessionJwt,
    AppState,
};

pub struct ClientAuthGuard;

#[async_trait]
impl<S> FromRequestParts<S> for ClientAuthGuard
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        tracing::trace!(method = "from_request_parts",);

        let SessionJwt(session) = SessionJwt::from_request_parts(&mut *parts, state).await?;

        // validate session
        let app_state = AppState::from_ref(state);
        let db_context = &app_state.db_context;
        let session_repository = &*app_state.repository_container.as_ref().session_repository;
        SessionService::get_session(
            db_context,
            session_repository,
            &session.user_id,
            session.id.as_str(),
        )
        .await
        .map_err(|err| {
            tracing::debug!("session not found: {:?}", err);
            StatusCode::UNAUTHORIZED
        })?;

        // validate user/permissions
        let Path(path_client_id) = Path::<(String,)>::from_request_parts(&mut *parts, state)
            .await
            .map_err(|err| {
                tracing::debug!("bad path wildcard: {:?}", err);
                StatusCode::NOT_FOUND
            })?;

        let client_repository = &*app_state.repository_container.as_ref().client_repository;
        let client = ClientService::get_client_by_id(
            db_context,
            client_repository,
            path_client_id.0.as_str(),
        )
        .await
        .map_err(|err| {
            tracing::debug!("client not found: {:?}", err);
            StatusCode::NOT_FOUND
        })?;

        if session.user_id != client.user_id {
            tracing::debug!("user in auth does not match user in client");
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(Self)
    }
}
