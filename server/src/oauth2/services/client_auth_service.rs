use crate::{models::ClientModel, repositories::ClientRepository};

pub struct ClientAuthService;

impl ClientAuthService {
    pub async fn verify_credentials(
        client_repository: &Box<dyn ClientRepository>,
        id: &str,
        secret: &Option<String>,
    ) -> Result<ClientModel, ClientAuthServiceError> {
        client_repository
            .get_by_credentials(id, secret)
            .await
            .map_err(|_| ClientAuthServiceError::NotFound)
    }
}

pub enum ClientAuthServiceError {
    NotFound,
}
