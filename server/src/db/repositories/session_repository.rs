use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{auth::models::SessionModel, DbContext};

pub type SessionRepositoryArc = Arc<Box<dyn SessionRepository>>;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        session: &SessionModel,
    ) -> Result<SessionModel, SessionRepositoryError>;
    async fn get_by_hash(
        &self,
        db_context: &Arc<DbContext>,
        session_id: &str,
        user_id: &Uuid,
    ) -> Result<SessionModel, SessionRepositoryError>;
    async fn update(
        &self,
        db_context: &Arc<DbContext>,
        session: &SessionModel,
    ) -> Result<SessionModel, SessionRepositoryError>;
    async fn delete_by_user_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<(), SessionRepositoryError>;
}

pub enum SessionRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    NotUpdated,
    BadData,
    BadDelete,
}
