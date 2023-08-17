use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};
use uuid::Uuid;

use crate::{
    api::v1::services::SessionService, services::RedirectService, utils::extractors::SessionJwt,
    AppState,
};

pub struct RedirectAuthGuard;

#[async_trait]
impl<S> FromRequestParts<S> for RedirectAuthGuard
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
        let Path(path_redirect_id) = Path::<(Uuid,)>::from_request_parts(&mut *parts, state)
            .await
            .map_err(|err| {
                tracing::debug!("bad path wildcard: {:?}", err);
                StatusCode::NOT_FOUND
            })?;

        let redirect_repository = &*app_state.repository_container.as_ref().redirect_repository;
        let user_id = RedirectService::get_user_id_from_redirect_id(
            db_context,
            redirect_repository,
            &path_redirect_id.0,
        )
        .await
        .map_err(|err| {
            tracing::debug!("redirect not found: {:?}", err);
            StatusCode::NOT_FOUND
        })?;

        if session.user_id != user_id {
            tracing::debug!("user in auth does not match user in redirect");
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(Self)
    }
}
