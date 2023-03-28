use axum::{
    http::StatusCode,
    response::{IntoResponse, Response, Redirect},
};
use url::Url;

// Self::InvalidRequest                => "invalid_request",
// Self::AccessDenied(_)               => "access_denied",
// Self::ServerError(_)                => "server_error",
// Self::TemporarilyUnavailable(_)     => "temporary_error",
// Self::InvalidClientId               => "invalid_client",
// Self::InvalidRedirectUri            => "invalid_redirect",
// Self::UnsupportedResponseType(_)    => "unsupported_response_type",
// Self::InvalidScope(_)               => "invalid_scope",
// Self::UnsupportedGrantType          => "unsupported_grant_type",
// Self::BadVerificationCode           => "bad_verification_code",
// Self::ExpiredToken                  => "expired_token",
// Self::InvalidGrant                  => "invalid_grant",
// _                                   => "unknown",

pub trait ApiRejection {
    fn status_code(&self) -> StatusCode;
    fn error_code(&self) -> &'static str;
    fn error_description(&self) -> &'static str;

    fn to_response(&self, redirect_uri: &Option<Url>) -> Response {   
        match redirect_uri {
            Some(url) => {
                let mut url = url.clone();
                url.query_pairs_mut().append_pair("error", self.error_code());
                Redirect::to(url.as_str()).into_response()
            },
            None => {
                (
                    self.status_code(), 
                    format!("error={};error_description={}", self.error_code(), self.error_description()),
                ).into_response()
            },
        }
    }
}

pub enum ServerRejection {
    ServerError,
    TemporarilyUnavailable,
}

impl ApiRejection for ServerRejection {
    fn status_code(&self) -> StatusCode {
        match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            Self::ServerError               => "server_error",
            Self::TemporarilyUnavailable    => "temporary_error",
        }
    }

    fn error_description(&self) -> &'static str {
        match self {
            Self::ServerError               => "An internal error has occurred, please try again.",
            Self::TemporarilyUnavailable    => "The route you are attempting to access is temporarily unavailable. Please try again later.",
        }
    }
}

pub type ApiResponse = std::result::Result<Response, Response>;

