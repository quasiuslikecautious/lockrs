use diesel_async::AsyncPgConnection;

use crate::auth::models::SessionModel;

pub struct SessionService;

impl SessionService {
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
