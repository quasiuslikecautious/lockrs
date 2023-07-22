use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use ring::rand::{SecureRandom, SystemRandom};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    db::{
        repositories::{AccessTokenRepository, RefreshTokenRepository},
        DbContext,
    },
    oauth2::v1::{
        models::{AccessTokenCreateModel, RefreshTokenCreateModel, ScopeModel, TokenModel},
        services::{
            AccessTokenService, AccessTokenServiceError, RefreshTokenService,
            RefreshTokenServiceError,
        },
    },
};

pub struct TokenService;

impl TokenService {
    pub async fn create_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        refresh_token_repository: &dyn RefreshTokenRepository,
        client_id: &str,
        user_id: Option<&Uuid>,
        scopes: ScopeModel,
    ) -> Result<TokenModel, TokenServiceError> {
        let access_expiry = (Utc::now() + Duration::minutes(10)).naive_utc();

        let access_token_create = AccessTokenCreateModel {
            token: Self::generate_opaque_token()?,
            client_id: client_id.to_string(),
            user_id: user_id.cloned(),
            expires_at: access_expiry,
            scopes: scopes.scopes.clone(),
        };

        let access_token = AccessTokenService::create_token(
            db_context,
            access_token_repository,
            &access_token_create,
        )
        .await
        .map_err(TokenServiceError::from)?;

        let refresh_expiry = (Utc::now() + Duration::hours(24)).naive_utc();

        let refresh_token_create = RefreshTokenCreateModel {
            access_token_id: access_token.id,
            token: Self::generate_opaque_token()?,
            client_id: client_id.to_string(),
            user_id: user_id.cloned(),
            expires_at: refresh_expiry,
            scopes: scopes.scopes.clone(),
        };

        let refresh_token = RefreshTokenService::create_token(
            db_context,
            refresh_token_repository,
            &refresh_token_create,
        )
        .await
        .map_err(TokenServiceError::from)?;

        Ok(TokenModel {
            token_type: String::from("Bearer"),
            expires_in: 5000,
            access_token: access_token.token,
            refresh_token: refresh_token.token,
            scopes: scopes
                .scopes
                .into_iter()
                .fold(String::new(), |c, s| format!("{} {}", c, s)),
        })
    }

    pub fn generate_opaque_token() -> Result<String, TokenServiceError> {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).map_err(|_| {
            TokenServiceError::InternalError(
                "ring::SystemRandom::fill failed on generate_opaque_token".into(),
            )
        })?;
        Ok(general_purpose::URL_SAFE_NO_PAD.encode(buffer))
    }
}

#[derive(Debug, Error)]
pub enum TokenServiceError {
    #[error("TOKEN SERVICE ERROR :: Token Not Created :: {0} ")]
    NotCreated(String),

    #[error("TOKEN SERVICE ERROR :: Internal Error :: {0}")]
    InternalError(String),
}

impl From<AccessTokenServiceError> for TokenServiceError {
    fn from(err: AccessTokenServiceError) -> Self {
        match err {
            AccessTokenServiceError::NotCreated(msg) => Self::NotCreated(msg),

            AccessTokenServiceError::NotFound(msg) => Self::InternalError(msg),
            AccessTokenServiceError::NotDeleted(msg) => Self::InternalError(msg),
            AccessTokenServiceError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}

impl From<RefreshTokenServiceError> for TokenServiceError {
    fn from(err: RefreshTokenServiceError) -> Self {
        match err {
            RefreshTokenServiceError::NotCreated(msg) => Self::NotCreated(msg),

            RefreshTokenServiceError::NotFound(msg) => Self::InternalError(msg),
            RefreshTokenServiceError::NotUpdated(msg) => Self::InternalError(msg),
            RefreshTokenServiceError::NotDeleted(msg) => Self::InternalError(msg),
            RefreshTokenServiceError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}
