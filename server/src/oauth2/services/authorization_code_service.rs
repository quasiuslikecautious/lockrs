use crate::oauth2::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel};

pub struct AuthorizationCodeService;

impl AuthorizationCodeService {
    pub fn create(new_code: AuthorizationCodeCreateModel) -> AuthorizationCodeModel {
        todo!();
    }
}

pub enum AuthorizationCodeServiceError {
    DbError,
}

impl From<diesel::result::Error> for AuthorizationCodeServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            _ => Self::DbError,
        }
    }
}
