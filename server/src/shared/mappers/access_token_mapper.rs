use crate::{db::models::DbAccessToken, oauth2::models::AccessToken};

use super::ScopeMapper;

pub struct AccessTokenMapper;

impl AccessTokenMapper {
    pub fn from_db(db_token: DbAccessToken) -> AccessToken {
        AccessToken {
            token: db_token.token,
            client_id: db_token.client_id,
            user_id: db_token.user_id,
            scopes: ScopeMapper::db_list_to_vec(&db_token.scopes),
            expires_at: db_token.expires_at,
        }
    }
}

