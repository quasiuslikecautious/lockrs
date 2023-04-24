use diesel_async::AsyncPgConnection;
use uuid::Uuid;

use crate::auth::models::{SessionModel, SessionCreateModel};

pub struct SessionService;

impl SessionService {
    pub async fn create_session(
        // redis connection
        _token: SessionCreateModel,
    ) -> Result<SessionModel, SessionServiceError> {
        todo!();
    }

    pub async fn get_session_by_id(
        // redis connection
        id: &str,
    ) -> Result<SessionModel, SessionServiceError> {
        todo!();
    }

    pub async fn get_sessions_by_user_id(
        // redis connection?
        user_id: &Uuid,
    ) -> Result<SessionModel, SessionServiceError> {
        todo!();
    }

    pub async fn update_session_by_id(
        // redis connection
        id: &str,
    ) -> Result<SessionModel, SessionServiceError> {
        todo!();
    }

    pub async fn refresh_session_by_id(
        // redis connection 
        id: &str,
    ) -> Result<SessionModel, SessionServiceError> {
        todo!();
    }

    pub async fn delete_session(
        _connection: &mut AsyncPgConnection,
        _session_id: &str,
    ) -> Result<SessionModel, SessionServiceError> {
        todo!();
    }
}

pub enum SessionServiceError {
    NotFound,
}
