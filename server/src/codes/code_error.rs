use axum::http::StatusCode;

use crate::api_response::ApiRejection;

pub enum CodeRejection {
    // combined
    InvalidClientId,
    MissingClientId,
    InvalidScope,
    NoScope,

    // authorization code specific
    InvalidRedirectUri,
    NoRedirectUri,
    InvalidResponseType,
    NoResponseType,
    NoState,
    AccessDenied,

    // PKCE Specific
    InvalidCodeChallenge,
    NoCodeChallenge,
    InvalidCodeChallengeMethod,
    NoCodeChallengeMethod,
}

impl ApiRejection for CodeRejection {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidClientId |
            Self::AccessDenied          => StatusCode::UNAUTHORIZED,
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
            Self::InvalidRedirectUri            => "The provided redirect uri is either not a valid url (absolute paths are required), or not registered with the specified client id.",
            Self::NoRedirectUri                 => "The field \"redirect_uri\" is missing from the query parameters of the request.",
            Self::InvalidClientId               => "The provided client identifier is either not in a valid identifier format, or is not recognized by the authentication server.",
            Self::MissingClientId               => "The field \"client_id\" is missing from the query parameters of the request.",

            Self::InvalidResponseType           => "The requested response type is not supported by the authorization server. Only the \"code\" response type is supported.",
            Self::NoResponseType                => "The field \"response_type\" is missing from the query parameters of the request.",
            Self::InvalidScope                  => "The requested scope(s) is not recognized as a valid scope with the authorization server.",
            Self::NoScope                       => "360",
            Self::NoState                       => "The field \"state\" is missing from the query parameters of the request.",
            Self::InvalidCodeChallenge          => "The provided code challenge is not in a valid format. The challenge must be uri safe, and between 43 and 128 characters.",
            Self::NoCodeChallenge               => "The field \"code_challenge\" is missing from the query parameters of the request.",
            Self::InvalidCodeChallengeMethod    => "The requested code challenge method is not supported by the authorization server. Please choose from the supported methods, [S256, plain]",
            Self::NoCodeChallengeMethod         => "The field \"code_challenge_method\" is missing from the query parameters of the request.",
            Self::AccessDenied                  => "The authorization request has been denied by the user.",
        }
    }
}

