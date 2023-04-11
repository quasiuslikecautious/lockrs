use crate::{db::models::DbRefreshToken, oauth2::models::RefreshToken};

use super::ScopeMapper;

pub struct RefreshTokenMapper;

impl RefreshTokenMapper {
    pub fn from_db(db_token: DbRefreshToken) -> RefreshToken{
        RefreshToken {
            token: db_token.token,
            user_id: db_token.user_id,
            client_id: db_token.client_id,
            scopes: ScopeMapper::db_list_to_vec(&db_token.scopes),
            expires_at: db_token.expires_at,
        }
    }
}

