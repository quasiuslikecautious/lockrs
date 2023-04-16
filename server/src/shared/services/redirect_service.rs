use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use url::Url;

use crate::{
    db::{models::DbRedirectUri, schema::redirect_uris},
    mappers::RedirectMapper,
    models::RedirectModel,
};

pub struct RedirectService;

impl RedirectService {
    pub async fn get_redirects_from_client(
        connection: &mut AsyncPgConnection,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RedirectServiceError> {
        let db_redirects = redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .load::<DbRedirectUri>(connection)
            .await
            .map_err(RedirectServiceError::from)?;

        Ok(db_redirects
            .into_iter()
            .map(RedirectMapper::from_db)
            .collect::<Vec<RedirectModel>>())
    }

    pub async fn verify_redirect(
        connection: &mut AsyncPgConnection,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectServiceError> {
        let db_redirect = redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .filter(redirect_uris::uri.eq(uri.to_string()))
            .first::<DbRedirectUri>(connection)
            .await
            .map_err(RedirectServiceError::from)?;

        Ok(RedirectMapper::from_db(db_redirect))
    }

    pub async fn create_redirect(
        connection: &mut AsyncPgConnection,
        client_id: &String,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectServiceError> {
        let db_redirect = diesel::insert_into(redirect_uris::table)
            .values((
                redirect_uris::client_id.eq(client_id),
                redirect_uris::uri.eq(uri.to_string()),
            ))
            .get_result::<DbRedirectUri>(connection)
            .await
            .map_err(|_| RedirectServiceError::DbError)?;

        Ok(RedirectMapper::from_db(db_redirect))
    }
}

pub enum RedirectServiceError {
    DbError,
    NotFound,
}

impl From<diesel::result::Error> for RedirectServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::NotFound,
            _ => Self::DbError,
        }
    }
}
