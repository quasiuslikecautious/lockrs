use std::sync::Arc;

use crate::{
    db::{repositories::AuthorizationCodeRepository, DbContext},
    oauth2::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel},
};

pub struct AuthorizationCodeService;

impl AuthorizationCodeService {
    pub async fn create(
        _db_context: &Arc<DbContext>,
        _authorization_code_repository: &dyn AuthorizationCodeRepository,
        _new_code: AuthorizationCodeCreateModel,
    ) -> AuthorizationCodeModel {
        todo!();
    }
}

pub enum AuthorizationCodeServiceError {
    NotCreated,
}
