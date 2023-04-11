use diesel::prelude::*;
use url::Url;

use crate::{
    db::{
        establish_connection,
        models::DbRedirectUri,
        schema::redirect_uris,
    },
    models::RedirectUri, mappers::RedirectMapper, 
};

pub struct RedirectService;

impl RedirectService {
    pub fn get_redirects_from_client(client_id: &str) -> Result<Vec<RedirectUri>, RedirectServiceError> {
        let connection = &mut establish_connection();
        let db_redirects = redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .load::<DbRedirectUri>(connection)
        .map_err(|err| {
            match err {
                diesel::result::Error::NotFound => RedirectServiceError::NotFound,
                _ => RedirectServiceError::DbError,
            }
        })?;

        Ok(
            db_redirects.into_iter().map(|x| RedirectMapper::from_db(x)).collect::<Vec<RedirectUri>>()
        )
    }

    pub fn verify_redirect(client_id: &str, uri: &Url) -> Result<RedirectUri, RedirectServiceError> {

        println!("{}", &uri.to_string());

        let connection = &mut establish_connection();
        let db_redirect = redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .filter(redirect_uris::uri.eq(uri.to_string()))
            .first::<DbRedirectUri>(connection)
        .map_err(|err| {
            match err {
                diesel::result::Error::NotFound => RedirectServiceError::NotFound,
                _ => RedirectServiceError::DbError,
            }
        })?;

        Ok(RedirectMapper::from_db(db_redirect))
    }

    pub fn create_redirect(client_id: &String, uri: &Url) -> Result<RedirectUri, RedirectServiceError> {
        let connection = &mut establish_connection();
        let db_redirect = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(redirect_uris::table)
                    .values((
                        redirect_uris::client_id.eq(client_id),
                        redirect_uris::uri.eq(uri.to_string())
                    ))
                    .get_result::<DbRedirectUri>(conn)
            })
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

