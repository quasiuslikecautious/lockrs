use std::{ops::Deref, sync::Arc};

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
        tracing::trace!(method = "create_token", client_id, ?user_id, ?scopes);

        let access_expiry = (Utc::now() + Duration::minutes(10)).naive_utc();

        let access_token_create = AccessTokenCreateModel::new(
            Self::generate_opaque_token()?.as_str(),
            client_id,
            user_id,
            &access_expiry,
            scopes.deref(),
        );

        let access_token = AccessTokenService::create_token(
            db_context,
            access_token_repository,
            &access_token_create,
        )
        .await
        .map_err(TokenServiceError::from)?;

        let refresh_expiry = (Utc::now() + Duration::hours(24)).naive_utc();

        let refresh_token_create = RefreshTokenCreateModel::new(
            Self::generate_opaque_token()?.as_str(),
            access_token.id,
            client_id,
            user_id,
            &refresh_expiry,
            scopes.deref(),
        );

        let refresh_token = RefreshTokenService::create_token(
            db_context,
            refresh_token_repository,
            &refresh_token_create,
        )
        .await
        .map_err(TokenServiceError::from)?;

        let token = TokenModel::new(
            "Bearer",
            5000,
            access_token.token.as_str(),
            refresh_token.token.as_str(),
            scopes
                .deref()
                .iter()
                .fold(String::new(), |c, s| format!("{} {}", c, s))
                .as_str(),
        );

        tracing::info!(
            "Token created: {{ client_id: {}, scopes: {:?} }}",
            client_id,
            &token.scopes
        );

        Ok(token)
    }

    pub fn generate_opaque_token() -> Result<String, TokenServiceError> {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).map_err(|_| {
            let msg = "ring::SystemRandom::fill failed on generate_opaque_token";

            tracing::error!(error = msg);
            TokenServiceError::InternalError
        })?;
        Ok(general_purpose::URL_SAFE_NO_PAD.encode(buffer))
    }
}

#[derive(Debug, Error)]
pub enum TokenServiceError {
    #[error("TOKEN SERVICE ERROR :: Token Not Created")]
    NotCreated,

    #[error("TOKEN SERVICE ERROR :: Internal Error")]
    InternalError,
}

impl From<AccessTokenServiceError> for TokenServiceError {
    fn from(err: AccessTokenServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            AccessTokenServiceError::NotCreated => Self::NotCreated,

            AccessTokenServiceError::NotFound => Self::InternalError,
            AccessTokenServiceError::NotDeleted => Self::InternalError,
            AccessTokenServiceError::InternalError => Self::InternalError,
        }
    }
}

impl From<RefreshTokenServiceError> for TokenServiceError {
    fn from(err: RefreshTokenServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            RefreshTokenServiceError::NotCreated => Self::NotCreated,

            RefreshTokenServiceError::NotFound => Self::InternalError,
            RefreshTokenServiceError::NotUpdated => Self::InternalError,
            RefreshTokenServiceError::NotDeleted => Self::InternalError,
            RefreshTokenServiceError::InternalError => Self::InternalError,
        }
    }
}
