use url::Url;

use crate::{
    models::{RedirectCreateModel, RedirectModel},
    repositories::RedirectUriRepository,
};

pub struct RedirectService;

impl RedirectService {
    pub async fn create_redirect(
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &String,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectServiceError> {
        let redirect_create = RedirectCreateModel {
            client_id: client_id.clone(),
            uri: uri.clone(),
        };

        redirect_repository
            .create(&redirect_create)
            .await
            .map_err(|_| RedirectServiceError::NotCreated)
    }

    pub async fn verify_redirect(
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectServiceError> {
        redirect_repository
            .get_by_uri(client_id, uri)
            .await
            .map_err(|_| RedirectServiceError::NotFound)
    }

    pub async fn get_redirects_from_client(
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RedirectServiceError> {
        redirect_repository
            .get_all_by_client_id(client_id)
            .await
            .map_err(|_| RedirectServiceError::NoneFound)
    }
}

pub enum RedirectServiceError {
    NotCreated,
    NotFound,
    NoneFound,
}
