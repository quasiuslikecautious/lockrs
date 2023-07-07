use std::sync::Arc;

use crate::{
    oauth2::models::{AccessTokenCreateModel, AccessTokenModel},
    repositories::AccessTokenRepository,
    DbContext,
};

pub struct AccessTokenService {}

impl AccessTokenService {
    pub async fn create_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        token_create: &AccessTokenCreateModel,
    ) -> Result<AccessTokenModel, AccessTokenServiceError> {
        access_token_repository
            .create(db_context, token_create)
            .await
            .map_err(|_| AccessTokenServiceError::NotCreated)
    }

    pub async fn verify_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        token: &str,
    ) -> Result<AccessTokenModel, AccessTokenServiceError> {
        access_token_repository
            .get_by_token(db_context, token)
            .await
            .map_err(|_| AccessTokenServiceError::NotFound)
    }

    pub async fn delete_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        token: &str,
    ) -> Result<(), AccessTokenServiceError> {
        access_token_repository
            .delete_by_token(db_context, token)
            .await
            .map_err(|_| AccessTokenServiceError::BadDelete)
    }
}

pub enum AccessTokenServiceError {
    NotCreated,
    NotFound,
    BadDelete,
}
