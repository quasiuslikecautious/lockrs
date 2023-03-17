use diesel::prelude::*;
use url::Url;

use crate::{auth_response, db, models, schema};

pub struct RedirectUri {
    client: models::ValidatedClient,
}

impl RedirectUri {
    pub fn new(client: &models::ValidatedClient) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn validate(&self, redirect_uri: &Url) -> auth_response::Result<()> {
        use schema::redirect_uris;

        let connection = &mut db::establish_connection();
        connection.build_transaction()
            .read_only()
            .run(|conn| {
                redirect_uris::table
                    .filter(redirect_uris::client_id.eq(&self.client.get_id()))
                    .filter(redirect_uris::uri.eq(&redirect_uri.as_str()))
                    .first::<db::DbRedirectUri>(conn)
            })
            .map_err(|_| auth_response::Rejection::InvalidRedirectUri)?;

        return Ok(());
    }
}