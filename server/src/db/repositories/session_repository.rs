use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    auth::models::SessionModel,
    db::{repositories::RepositoryError, DbContext},
};

pub type SessionRepositoryArc = Arc<Box<dyn SessionRepository>>;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        session: &SessionModel,
    ) -> Result<SessionModel, RepositoryError>;
    async fn get_by_hash(
        &self,
        db_context: &Arc<DbContext>,
        session_id: &str,
        user_id: &Uuid,
    ) -> Result<SessionModel, RepositoryError>;
    async fn update(
        &self,
        db_context: &Arc<DbContext>,
        session: &SessionModel,
    ) -> Result<SessionModel, RepositoryError>;
    async fn delete_by_user_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<(), RepositoryError>;
}
