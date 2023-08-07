use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use headers::{authorization::Bearer, Authorization, HeaderMapExt};

#[derive(Debug)]
pub struct BearerAuth(pub String);

#[async_trait()]
impl<S> FromRequestParts<S> for BearerAuth
where
    S: Send + Sync,
{
    type Rejection = BearerAuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bearer = parts
            .headers
            .typed_get::<Authorization<Bearer>>()
            .ok_or(Self::Rejection::InvalidHeader)?;

        Ok(BearerAuth(String::from(bearer.token())))
    }
}

pub enum BearerAuthError {
    InvalidHeader,
}

impl BearerAuthError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InvalidHeader => "Invalid authorization header.",
        }
    }
}

impl IntoResponse for BearerAuthError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
