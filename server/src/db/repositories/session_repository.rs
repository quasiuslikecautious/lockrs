use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::auth::models::SessionModel;

pub type SessionRepositoryArc = Arc<Box<dyn SessionRepository>>;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: &SessionModel) -> Result<SessionModel, SessionRepositoryError>;
    async fn get_by_hash(
        &self,
        session_id: &str,
        user_id: &Uuid,
    ) -> Result<SessionModel, SessionRepositoryError>;
    async fn update(&self, session: &SessionModel) -> Result<SessionModel, SessionRepositoryError>;
    async fn delete_by_user_id(&self, id: &Uuid) -> Result<(), SessionRepositoryError>;
}

pub enum SessionRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    NotUpdated,
    BadData,
    BadDelete,
}
