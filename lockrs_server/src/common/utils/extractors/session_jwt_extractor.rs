use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};

use crate::{
    api::v1::models::SessionModel,
    utils::{extractors::Cookies, jwt::JwtUtil},
    AppState,
};

#[derive(Debug)]
pub struct SessionJwt(pub SessionModel);

#[async_trait]
impl<S> FromRequestParts<S> for SessionJwt
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Cookies(cookies) = Cookies::from_request_parts(&mut *parts, state)
            .await
            .map_err(|err| {
                tracing::debug!("missing cookies: {:?}", err);
                StatusCode::UNAUTHORIZED
            })?;

        let Some(jwt) = cookies.get(JwtUtil::cookie_name())
        else {
            tracing::debug!("missing session jwt cookie");
            return Err(StatusCode::UNAUTHORIZED);
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

        Ok(Self(claims.claims))
    }
}
