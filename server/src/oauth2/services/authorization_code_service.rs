use url::Url;

use crate::oauth2::models::Scopes;

pub struct AuthorizationCodeService;

impl AuthorizationCodeService {
    pub fn create(
        client_id: &str,
        user_id: &str,
        code: &str,
        is_plain: bool,
        redirect_uri: Url,
        scopes: Scopes,
    ) {

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

