use std::sync::Arc;

use crate::{models::ClientModel, repositories::ClientRepository, DbContext};

pub struct ClientAuthService;

impl ClientAuthService {
    pub async fn verify_credentials(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        id: &str,
        secret: Option<&str>,
    ) -> Result<ClientModel, ClientAuthServiceError> {
        client_repository
            .get_by_credentials(db_context, id, secret)
            .await
            .map_err(|_| ClientAuthServiceError::NotFound)
    }
}

pub enum ClientAuthServiceError {
    NotFound,
}
