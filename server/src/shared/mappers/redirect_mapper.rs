use url::Url;

use crate::{db::models::DbRedirectUri, models::RedirectUri};

pub struct RedirectMapper;

impl RedirectMapper {
    pub fn from_db(db_redirect: DbRedirectUri) -> RedirectUri {
        RedirectUri {
            id: db_redirect.id,
            client_id: db_redirect.client_id,
            uri: Url::parse(&db_redirect.uri).expect(format!("invalid url stored in database: {}", db_redirect.id).as_str()),
        }
    }
}

