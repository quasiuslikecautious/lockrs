use crate::{
    oauth2::models::{RefreshTokenCreateModel, RefreshTokenModel},
    repositories::RefreshTokenRepository,
};

pub struct RefreshTokenService {}

impl RefreshTokenService {
    pub async fn create_token(
        refresh_token_repository: &dyn RefreshTokenRepository,
        token_create: &RefreshTokenCreateModel,
    ) -> Result<RefreshTokenModel, RefreshTokenServiceError> {
        refresh_token_repository
            .create(token_create)
            .await
            .map_err(|_| RefreshTokenServiceError::NotCreated)
    }

    pub async fn get_by_token(
        refresh_token_repository: &dyn RefreshTokenRepository,
        token: &str,
    ) -> Result<RefreshTokenModel, RefreshTokenServiceError> {
        refresh_token_repository
            .get_by_token(token)
            .await
            .map_err(|_| RefreshTokenServiceError::NotFound)
    }

    pub async fn use_token(
        refresh_token_repository: &dyn RefreshTokenRepository,
        token: &str,
    ) -> Result<RefreshTokenModel, RefreshTokenServiceError> {
        refresh_token_repository
            .use_by_token(token)
            .await
            .map_err(|_| RefreshTokenServiceError::NotUsed)
    }

    pub async fn delete_token(
        refresh_token_repository: &dyn RefreshTokenRepository,
        token: &str,
    ) -> Result<(), RefreshTokenServiceError> {
        refresh_token_repository
            .delete_by_token(token)
            .await
            .map_err(|_| RefreshTokenServiceError::BadDelete)
    }
}

pub enum RefreshTokenServiceError {
    NotCreated,
    NotFound,
    NotUsed,
    BadDelete,
}
