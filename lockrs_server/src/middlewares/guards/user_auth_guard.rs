use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};
use uuid::Uuid;

use crate::{
    api::v1::{models::SessionModel, services::SessionService},
    utils::extractors::Cookies,
    AppState,
};

pub struct UserAuthGuard;

#[async_trait]
impl<S> FromRequestParts<S> for UserAuthGuard
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        tracing::trace!(method = "from_request_parts",);

        let Cookies(cookies) = Cookies::from_request_parts(&mut *parts, state)
            .await
            .map_err(|err| {
                tracing::debug!("missing cookies: {:?}", err);
                StatusCode::BAD_REQUEST
            })?;

        let Some(jwt) = cookies.get("s_jwt")
        else {
            tracing::debug!("missing session jwt cookie");
            return Err(StatusCode::BAD_REQUEST);
        };

        // validate jwt
        let app_state = AppState::from_ref(state);
        let claims = app_state
            .jwt_util
            .verify_jwt::<SessionModel>(jwt)
            .map_err(|err| {
                tracing::debug!("bad jwt cookie: {:?}", err);
                StatusCode::UNAUTHORIZED
            })?;

        // validate session
        let db_context = &app_state.db_context;
        let session_repository = &*app_state.repository_container.as_ref().session_repository;
        SessionService::get_session(
            db_context,
            session_repository,
            &claims.claims.user_id,
            claims.claims.id.as_str(),
        )
        .await
        .map_err(|err| {
            tracing::debug!("session not found: {:?}", err);
            StatusCode::UNAUTHORIZED
        })?;

        // validate user/permissions
        let Path(path_user_id) = Path::<(Uuid,)>::from_request_parts(&mut *parts, state)
            .await
            .map_err(|err| {
                tracing::debug!("bad path wildcard: {:?}", err);
                StatusCode::NOT_FOUND
            })?;

        if claims.claims.user_id != path_user_id.0 {
            tracing::debug!("user in auth does not match user in path");
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(Self)
    }
}
