use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use headers::{HeaderMapExt, Cookie};


#[derive(Debug)]
pub struct Cookies(pub Cookie);

impl Cookies {
    pub fn extract<S>(parts: &Parts, _state: &S) -> Result<Self, CookieError>
    where
        S: Send + Sync,
    {
        let cookies = parts
            .headers
            .typed_get::<Cookie>()
            .ok_or(CookieError::MissingCookie)?;

        Ok(Cookies(cookies))
    }
}

#[async_trait()]
impl<S> FromRequestParts<S> for Cookies
where
    S: Send + Sync,
{
    type Rejection = CookieError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Self::extract(parts, state)
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
