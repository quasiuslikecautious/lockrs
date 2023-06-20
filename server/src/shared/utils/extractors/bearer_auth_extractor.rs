use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    response::IntoResponse,
};

#[derive(Debug)]
pub struct BearerAuth(pub String);

#[async_trait()]
impl<S> FromRequestParts<S> for BearerAuth
where
    S: Send + Sync,
{
    type Rejection = BearerAuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let Some(authorization) = headers.get(AUTHORIZATION)
        else {
            return Err(Self::Rejection::MissingAuthorizationHeader);
        };

        let Some(header_str) = authorization.to_str().ok()
        else {
            return Err(Self::Rejection::InvalidHeaderEncoding);
        };

        let Some((auth_header, token)) = header_str.split_once(' ')
        else {
            return Err(Self::Rejection::InvalidHeaderFormat);
        };

        if !auth_header.eq("Bearer") {
            return Err(Self::Rejection::InvalidAuthenticationType);
        }

        Ok(BearerAuth(String::from(token)))
    }
}

pub enum BearerAuthError {
    MissingAuthorizationHeader,
    InvalidHeaderEncoding,
    InvalidHeaderFormat,
    InvalidAuthenticationType,
}

impl BearerAuthError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::MissingAuthorizationHeader => "Missing authorization header from request.",
            Self::InvalidHeaderEncoding => "Invalid header encoding. Ensure your header is UTF-8 encoded.",
            Self::InvalidHeaderFormat => "Invalid header format. Ensure your authorization header is set to \"Bearer <token>.\"",
            Self::InvalidAuthenticationType => "Invalid authorization type. Please use \"Bearer.\"",
        }
    }
}

impl IntoResponse for BearerAuthError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
