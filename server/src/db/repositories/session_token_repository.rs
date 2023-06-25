use std::sync::Arc;

use async_trait::async_trait;

use crate::auth::models::SessionTokenModel;

pub type SessionTokenRepositoryArc = Arc<Box<dyn SessionTokenRepository>>;

#[async_trait]
pub trait SessionTokenRepository: Send + Sync {
    async fn create(
        &self,
        token: &SessionTokenModel,
    ) -> Result<SessionTokenModel, SessionTokenRepositoryError>;
    async fn get_by_token(
        &self,
        token: &str,
    ) -> Result<SessionTokenModel, SessionTokenRepositoryError>;
    async fn delete_by_token(&self, token: &str) -> Result<(), SessionTokenRepositoryError>;
}

pub enum SessionTokenRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    BadData,
    BadDelete,
}
