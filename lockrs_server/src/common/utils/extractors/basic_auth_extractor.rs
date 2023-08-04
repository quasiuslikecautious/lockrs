use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use headers::{authorization::Basic, Authorization, HeaderMapExt};

#[derive(Debug)]
pub struct BasicAuthCredentials {
    pub public: String,
    pub private: String,
}

#[derive(Debug)]
pub struct BasicAuth(pub BasicAuthCredentials);

#[async_trait()]
impl<S> FromRequestParts<S> for BasicAuth
where
    S: Send + Sync,
{
    type Rejection = BasicAuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let credentials = parts
            .headers
            .typed_get::<Authorization<Basic>>()
            .ok_or(Self::Rejection::InvalidHeader)?;

        Ok(BasicAuth(BasicAuthCredentials {
            public: credentials.username().to_string(),
            private: credentials.password().to_string(),
        }))
    }
}

pub enum BasicAuthError {
    InvalidHeader,
}

impl BasicAuthError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InvalidHeader => "Invalid authorization header",
        }
    }
}

impl IntoResponse for BasicAuthError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
