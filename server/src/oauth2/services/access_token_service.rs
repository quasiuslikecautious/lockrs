use crate::{
    oauth2::models::{AccessTokenCreateModel, AccessTokenModel},
    repositories::AccessTokenRepository,
};

pub struct AccessTokenService {}

impl AccessTokenService {
    pub async fn create_token(
        access_token_repository: &dyn AccessTokenRepository,
        token_create: &AccessTokenCreateModel,
    ) -> Result<AccessTokenModel, AccessTokenServiceError> {
        access_token_repository
            .create(token_create)
            .await
            .map_err(|_| AccessTokenServiceError::NotCreated)
    }

    pub async fn verify_token(
        access_token_repository: &dyn AccessTokenRepository,
        token: &str,
    ) -> Result<AccessTokenModel, AccessTokenServiceError> {
        access_token_repository
            .get_by_token(token)
            .await
            .map_err(|_| AccessTokenServiceError::NotFound)
    }

    pub async fn delete_token(
        access_token_repository: &dyn AccessTokenRepository,
        token: &str,
    ) -> Result<(), AccessTokenServiceError> {
        access_token_repository
            .delete_by_token(token)
            .await
            .map_err(|_| AccessTokenServiceError::BadDelete)
    }
}

pub enum AccessTokenServiceError {
    NotCreated,
    NotFound,
    BadDelete,
}
