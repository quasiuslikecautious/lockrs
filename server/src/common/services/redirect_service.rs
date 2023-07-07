use std::sync::Arc;

use url::Url;

use crate::{
    db::{repositories::RedirectUriRepository, DbContext},
    models::{RedirectCreateModel, RedirectModel},
};

pub struct RedirectService;

impl RedirectService {
    pub async fn create_redirect(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectServiceError> {
        let redirect_create = RedirectCreateModel {
            client_id: client_id.to_string(),
            uri: uri.clone(),
        };

        redirect_repository
            .create(db_context, &redirect_create)
            .await
            .map_err(|_| RedirectServiceError::NotCreated)
    }

    pub async fn verify_redirect(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectServiceError> {
        redirect_repository
            .get_by_uri(db_context, client_id, uri)
            .await
            .map_err(|_| RedirectServiceError::NotFound)
    }

    pub async fn get_redirects_from_client(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RedirectServiceError> {
        redirect_repository
            .get_all_by_client_id(db_context, client_id)
            .await
            .map_err(|_| RedirectServiceError::NoneFound)
    }
}

pub enum RedirectServiceError {
    NotCreated,
    NotFound,
    NoneFound,
}
