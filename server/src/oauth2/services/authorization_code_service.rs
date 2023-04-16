use diesel_async::AsyncPgConnection;

use crate::oauth2::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel};

pub struct AuthorizationCodeService;

impl AuthorizationCodeService {
    pub async fn create(
        _connection: &mut AsyncPgConnection,
        _new_code: AuthorizationCodeCreateModel,
    ) -> AuthorizationCodeModel {
        todo!();
    }
}

pub enum AuthorizationCodeServiceError {
    DbError,
}

impl From<diesel::result::Error> for AuthorizationCodeServiceError {
    fn from(_diesel_error: diesel::result::Error) -> Self {
        Self::DbError
    }
}
