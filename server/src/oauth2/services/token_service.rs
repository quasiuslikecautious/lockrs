use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use ring::rand::{SecureRandom, SystemRandom};
use uuid::Uuid;

use crate::{
    oauth2::models::{AccessTokenCreateModel, RefreshTokenCreateModel, ScopeModel, TokenModel},
    repositories::{AccessTokenRepository, RefreshTokenRepository},
};

pub struct TokenService;

impl TokenService {
    pub async fn create_token(
        access_token_repository: &Box<dyn AccessTokenRepository>,
        refresh_token_repository: &Box<dyn RefreshTokenRepository>,
        client_id: &str,
        user_id: &Option<Uuid>,
        scopes: ScopeModel,
    ) -> Result<TokenModel, TokenServiceError> {
        let access_expiry = (Utc::now() + Duration::minutes(10)).naive_utc();

        let access_token_create = AccessTokenCreateModel {
            token: Self::generate_opaque_token(),
            client_id: client_id.to_string(),
            user_id: user_id.clone(),
            expires_at: access_expiry,
            scopes: scopes.scopes.clone(),
        };

        let access_token = access_token_repository
            .create(&access_token_create)
            .await
            .map_err(|_| TokenServiceError::NotCreated)?;

        let refresh_expiry = (Utc::now() + Duration::hours(24)).naive_utc();

        let refresh_token_create = RefreshTokenCreateModel {
            access_token_id: access_token.id,
            token: Self::generate_opaque_token(),
            client_id: client_id.to_string(),
            user_id: user_id.clone(),
            expires_at: refresh_expiry,
            scopes: scopes.scopes.clone(),
        };

        let refresh_token = refresh_token_repository
            .create(&refresh_token_create)
            .await
            .map_err(|_| TokenServiceError::NotCreated)?;

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

    pub fn generate_opaque_token() -> String {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer)
    }
}

pub enum TokenServiceError {
    NotCreated,
    NotFound,
}
