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

#[async_trait]
impl<S> FromRequestParts<S> for SessionJwt
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = SessionJwtError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        todo!();
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
