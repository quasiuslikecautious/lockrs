use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    response::IntoResponse,
};
use base64::{engine::general_purpose, Engine as _};

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
        let headers = &parts.headers;

        let Some(authorization) = headers.get(AUTHORIZATION)
        else {
            return Err(Self::Rejection::MissingAuthorizationHeader);
        };

        let Some(header_str) = authorization.to_str().ok()
        else {
            return Err(Self::Rejection::InvalidHeaderEncoding);
        };

        let Some((auth_header, encoded_credentials)) = header_str.split_once(' ')
        else {
            return Err(Self::Rejection::InvalidHeaderFormat);
        };

        if !auth_header.eq("Basic") {
            return Err(Self::Rejection::InvalidAuthenticationType);
        }

        let Some(decoded_credentials_bytes) = general_purpose::STANDARD
            .decode(encoded_credentials).ok()
        else {
            return Err(Self::Rejection::InvalidAuthenticationParameterEncoding);
        };

        let Some(decoded_credentials) = String::from_utf8(decoded_credentials_bytes).ok()
        else {
            return Err(Self::Rejection::InvalidAuthenticationParameterEncoding);
        };

        let Some((public, private)) = decoded_credentials.split_once(':')
        else {
            return Err(Self::Rejection::InvalidAuthenticationParameterFormat);
        };

        Ok(BasicAuth(BasicAuthCredentials {
            public: public.to_string(),
            private: private.to_string(),
        }))
    }
}

pub enum BasicAuthError {
    MissingAuthorizationHeader,
    InvalidHeaderEncoding,
    InvalidHeaderFormat,
    InvalidAuthenticationType,
    InvalidAuthenticationParameterEncoding,
    InvalidAuthenticationParameterFormat,
}

impl BasicAuthError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::MissingAuthorizationHeader => "Missing authorization header from request.",
            Self::InvalidHeaderEncoding => "Invalid header encoding. Ensure your header is UTF-8 encoded.",
            Self::InvalidHeaderFormat => "Invalid header format. Ensure your authorization header is set to \"Basic <base64 encoded credentials>.\"",
            Self::InvalidAuthenticationType => "Invalid authorization type. Please use \"Basic.\"",
            Self::InvalidAuthenticationParameterEncoding => "Invalid authorization parameter encoding. Please ensure your credentials are a Base64 encoded UTF-8 string.",
            Self::InvalidAuthenticationParameterFormat => "Invalid authorization parameter format. Please ensure your unencoded credential string follows the format <public key>:<private key>."
        }
    }
}

impl IntoResponse for BasicAuthError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
