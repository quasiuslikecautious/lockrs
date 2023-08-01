use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};

use std::sync::Arc;

use crate::{api::v1::models::SessionModel, utils::extractors::Cookies, AppState};

#[derive(Debug)]
pub struct SessionJwt(pub SessionModel);

#[async_trait()]
impl<S> FromRequestParts<S> for SessionJwt
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = SessionJwtError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(Cookies(cookies)) = Cookies::from_request_parts(parts, state)
            .await
            .ok()
        else {
            return Err(SessionJwtError::CookieParsing);
        };

        let Some(jwt) = cookies.get(&String::from("s_jwt"))
        else {
            return Err(SessionJwtError::NotPresent);
        };

        let state = Arc::<AppState>::from_ref(state);

        let Some(claims) = Arc::as_ref(&state.jwt_util).verify_jwt::<SessionModel>(jwt.as_str()).ok()
        else {
            return Err(SessionJwtError::InvalidJwt);
        };

        Ok(SessionJwt(claims.claims))
    }
}

pub enum SessionJwtError {
    CookieParsing,
    NotPresent,
    InvalidJwt,
}

impl IntoResponse for SessionJwtError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::BAD_REQUEST.into_response()
    }
}