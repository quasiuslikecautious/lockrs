use crate::{oauth2::models::RefreshTokenModel, pg::models::PgRefreshToken};

use super::ScopeMapper;

pub struct RefreshTokenMapper;

impl RefreshTokenMapper {
    pub fn from_pg(pg_token: PgRefreshToken) -> RefreshTokenModel {
        RefreshTokenModel {
            id: pg_token.id,
            access_token_id: pg_token.access_token_id,
            token: pg_token.token,
            user_id: pg_token.user_id,
            client_id: pg_token.client_id,
            scopes: ScopeMapper::pg_list_to_vec(&pg_token.scopes),
            expires_at: pg_token.expires_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};
    use uuid::Uuid;

    #[test]
    fn it_should_map_pg_with_user() {
        let id = 1;
        let access_token_id = 2;
        let token = String::from("TOKEN");
        let client_id = String::from("CLIENT_ID");
        let user_id = Some(Uuid::new_v4());
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::minutes(1);
        let scopes = vec![Some(String::from("read")), Some(String::from("write"))];

        let pg_token = PgRefreshToken {
            id,
            access_token_id,
            token: token.clone(),
            client_id: client_id.clone(),
            user_id,
            created_at,
            expires_at,
            used: false,
            scopes,
        };

        let actual_token = RefreshTokenMapper::from_pg(pg_token);

        let expected_token = RefreshTokenModel {
            id,
            access_token_id,
            token,
            client_id,
            user_id,
            expires_at,
            scopes: vec![String::from("read"), String::from("write")],
        };

        assert_eq!(actual_token, expected_token);
    }

    #[test]
    fn it_should_map_pg_with_no_user() {
        let id = 1;
        let access_token_id = 2;
        let token = String::from("TOKEN");
        let client_id = String::from("CLIENT_ID");
        let user_id = None;
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::minutes(1);
        let scopes = vec![Some(String::from("read")), Some(String::from("write"))];

        let pg_token = PgRefreshToken {
            id,
            access_token_id,
            token: token.clone(),
            client_id: client_id.clone(),
            user_id,
            created_at,
            expires_at,
            used: false,
            scopes,
        };

        let actual_token = RefreshTokenMapper::from_pg(pg_token);

        let expected_token = RefreshTokenModel {
            id,
            access_token_id,
            token,
            client_id,
            user_id,
            expires_at,
            scopes: vec![String::from("read"), String::from("write")],
        };

        assert_eq!(actual_token, expected_token);
    }
}
