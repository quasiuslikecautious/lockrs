use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use headers::{Cookie, HeaderMapExt};

#[derive(Debug)]
pub struct Cookies(pub Cookie);

#[async_trait()]
impl<S> FromRequestParts<S> for Cookies
where
    S: Send + Sync,
{
    type Rejection = CookieError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .headers
            .typed_get::<Cookie>()
            .ok_or(CookieError::MissingCookie)?;

        Ok(Cookies(cookies))
    }
}

#[derive(Debug)]
pub enum CookieError {
    MissingCookie,
}

impl IntoResponse for CookieError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::BAD_REQUEST.into_response()
    }
}
