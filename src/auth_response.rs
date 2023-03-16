use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response, Redirect},
};
use serde::Serialize;
use url::Url;

#[derive(Serialize)]
pub struct ErrorMessage {
    pub error: String,
    pub error_desciption: String,
}

impl ErrorMessage {
    pub fn new(error: &str, error_description: &str) -> Self {
        Self {
            error: error.to_string(),
            error_desciption: error_description.to_string(),
        }
    }

    pub fn json(error: &str, error_description: &str) -> Json<Self> {
        Json(Self::new(error, error_description))
    }
}

#[derive(Debug)]
pub enum Rejection {
    Unknown,
    InvalidRequest,
    AccessDenied(Url),
    ServerError(Option<Url>),
    TemporarilyUnavailable(Url),
    InvalidClientId,
    InvalidRedirectUri,
    UnsupportedResponseType(Url),
    InvalidScope(Url),
    UnsupportedGrantType,
    BadVerificationCode,
    ExpiredToken,
}

impl From<&str> for Rejection {
    fn from(s: &str) -> Self {
        match s {
            "invalid_request"           => Self::InvalidRequest,
            "invalid_client"            => Self::InvalidClientId,
            "invalid_redirect"          => Self::InvalidRedirectUri,
            "unsupported_grant_type"    => Self::UnsupportedGrantType,
            "bad_verification_code"     => Self::BadVerificationCode,
            "expired_token"             => Self::ExpiredToken,
            _                           => Self::Unknown,
        }
    }
}

impl Rejection {
    pub fn into_callback_url(&self) -> String {
        let default_callback_url = String::from("http://127.0.0.1:8080/error");

        match self {
            Self::Unknown => default_callback_url,
            Self::InvalidRequest => default_callback_url,
            Self::AccessDenied(callback) => callback.to_string(),
            Self::ServerError(callback) => {
                match callback {
                    Some(redirect_uri) => redirect_uri.to_string(),
                    None => default_callback_url,
                }
            },
            Self::TemporarilyUnavailable(callback) => callback.to_string(),
            Self::InvalidClientId => default_callback_url,
            Self::InvalidRedirectUri => default_callback_url,
            Self::UnsupportedResponseType(callback) => callback.to_string(),
            Self::InvalidScope(callback) => callback.to_string(),
            Self::UnsupportedGrantType => default_callback_url,
            Self::BadVerificationCode => default_callback_url,
            Self::ExpiredToken => default_callback_url,
        }
    }

    pub fn into_error_code(&self) -> &'static str {
        match self {
            Self::InvalidRequest                => "invalid_request",
            Self::AccessDenied(_)               => "access_denied",
            Self::ServerError(_)                => "server_error",
            Self::TemporarilyUnavailable(_)     => "temporary_error",
            Self::InvalidClientId               => "invalid_client",
            Self::InvalidRedirectUri            => "invalid_redirect",
            Self::UnsupportedResponseType(_)    => "unsupported_response_type",
            Self::InvalidScope(_)               => "invalid_scope",
            Self::UnsupportedGrantType          => "unsupported_grant_type",
            Self::BadVerificationCode           => "bad_verification_code",
            Self::ExpiredToken                  => "expired_token",
            _                                   => "unknown",
        }
    }

    pub fn into_error_description(&self) -> &'static str {
        match self {
            Self::InvalidRequest                => "Invalid Request",
            Self::AccessDenied(_)               => "The resource owner has denied the authorization request",
            Self::ServerError(_)                => "An internal error occured while processing your request",
            Self::TemporarilyUnavailable(_)     => "Please try again later",
            Self::InvalidClientId               => "Invalid client_id supplied",
            Self::InvalidRedirectUri            => "Invalid redirect_uri supplied",
            Self::UnsupportedResponseType(_)    => "Unsupported response_type requested",
            Self::InvalidScope(_)               => "Invalid scope(s) requested",
            Self::UnsupportedGrantType          => "Unsupported grant type requested",
            Self::BadVerificationCode           => "The device code supplied is invalid",
            Self::ExpiredToken                  => "The device code supplied has expired",
            _                                   => "Unknown error code supplied"
        }
    }
}

impl IntoResponse for Rejection {
    fn into_response(self) -> Response {
        let callback = self.into_callback_url();
        let error_code = self.into_error_code();

        let redirect_uri = format!("{}?error={}", &callback, &error_code);

        Redirect::to(redirect_uri.as_str()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Rejection>;

pub enum Information {
    AuthorizationPending,
    SlowDown,
}

impl Information {
    pub fn into_error_code(&self) -> &'static str {
        match self {
            Self::AuthorizationPending  => "authorization_pending",
            Self::SlowDown              => "slow_down",
        }
    }
    
    
    pub fn into_error_description(&self) -> &'static str{
        match self {
            Self::AuthorizationPending  => "Authorization for user client is still pending",
            Self::SlowDown              => "Polling too fast, please slow down your request rate to match the interval specified.",
        }
    }
}

impl IntoResponse for Information {
    fn into_response(self) -> Response {
        let code = self.into_error_code();
        let description = self.into_error_description();
        let message = ErrorMessage::json(code, description);
        
        (StatusCode::BAD_REQUEST, message).into_response()
    }
}

