use axum::http::StatusCode;

use crate::api_response::ApiRejection;

pub enum TokenExchangeRejection {
    // general
    InvalidClientId,
    MissingClientId,
    InvalidGrantType,
    NoGrantType,
    
    // authorization code
    InvalidAuthCode,
    NoAuthCode,
    InvalidRedirectUri,
    NoRedirectUri,

    // device code
    InvalidDeviceCode,
    NoDeviceCode,

    // refresh token exchange
    InvalidRefreshToken,
    ExpiredRefreshToken,
    NoRefreshToken,
    InvalidScope,
}

impl ApiRejection for TokenExchangeRejection {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidClientId |
            Self::InvalidAuthCode |
            Self::InvalidDeviceCode |
            Self::InvalidRefreshToken |
            Self::ExpiredRefreshToken   => StatusCode::UNAUTHORIZED,
            _                           => StatusCode::BAD_REQUEST,
        }
    }
    
    fn error_code(&self) -> &'static str {
        match self {
            _ => "invalid_request",
        }
    }

    fn error_description(&self) -> &'static str {
        match self {
            Self::InvalidClientId               => "The provided client identifier is either not in a valid identifier format, or is not recognized by the authentication server.",
            Self::MissingClientId               => "The field \"client_id\" is missing from the query parameters of the request.",
            Self::InvalidGrantType              => "The requested grant type is not supported by the authorization server.",
            Self::NoGrantType                   => "The field \"grant_type\" is missing from the query parameters of the request.",
            Self::InvalidAuthCode               => "The provided authorization code is invalid.",
            Self::NoAuthCode                    => "The field \"code\" is missing from the query parameters of the request.",
            Self::InvalidRedirectUri            => "The provided redirect uri is either not a valid url (absolute paths are required), or not registered with the specified client id.",
            Self::NoRedirectUri                 => "The field \"redirect_uri\" is missing from the query parameters of the request.",
            Self::InvalidDeviceCode             => "The provided device code is invalid.",
            Self::NoDeviceCode                  => "The field \"device_code\" is missing from the query parameters of the request.",
            Self::InvalidRefreshToken           => "The provided refresh token is invalid.",
            Self::ExpiredRefreshToken           => "The provided refresh token is expired.",
            Self::NoRefreshToken                => "The field \"refresh_token\" is missing from the query parameters of the request.",
            Self::InvalidScope                  => "The provided refresh token is no longer sufficient for the scopes required.",
        }
    }
}

pub enum TokenVerificationRejection {
    // access token verification
    InvalidAccessToken,
    NoAccessToken,
    InvalidRefreshToken,
    NoRefreshToken,
    InvalidScope,
    TokenExpired,
    TokenRevoked,
}

impl ApiRejection for TokenVerificationRejection {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidScope  => StatusCode::FORBIDDEN,
            Self::NoAccessToken => StatusCode::BAD_REQUEST,
            _                   => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            Self::TokenExpired |
            Self::TokenRevoked |
            Self::InvalidScope          => "access_denied",
            Self::InvalidAccessToken |
            Self::InvalidRefreshToken |
            Self::NoAccessToken |
            Self::NoRefreshToken        => "invalid_request",
        }
    }

    fn error_description(&self) -> &'static str {
        match self {
            Self::InvalidAccessToken    => "The provided access token is invalid.",
            Self::NoAccessToken         => "The field \"access_token\" is missing from the query parameters of the request.",
            Self::InvalidRefreshToken   => "The provided refresh token is invalid.",
            Self::NoRefreshToken        => "The field \"refresh_token\" is missing from the query parameters of the request",
            Self::InvalidScope          => "The provided access token has insufficient access to the scopes required for the request.",
            Self::TokenExpired    => "The provided token has expired.",
            Self::TokenRevoked    => "The provided token has been revoked.",
        }
    }
}
