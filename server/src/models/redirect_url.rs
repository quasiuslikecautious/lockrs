use url::Url;

use crate::{
    auth_response,
    db::{DbError, models::DbRedirectUri},
    models
};

pub struct RedirectUri {

}

impl RedirectUri {
    pub fn validate(client: &models::Client, redirect_uri: &Url) -> auth_response::Result<()> {
        DbRedirectUri::get(&client.get_id(), redirect_uri)
            .map_err(|err| {
                match err {
                    DbError::NotFound       => auth_response::Rejection::InvalidRedirectUri,
                    DbError::InternalError  => auth_response::Rejection::ServerError(None),
                }
            })?;

        return Ok(());
    }
}
