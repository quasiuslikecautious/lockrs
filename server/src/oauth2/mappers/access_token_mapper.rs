use crate::{db::models::DbAccessToken, oauth2::models::AccessTokenModel};

use super::ScopeMapper;

pub struct AccessTokenMapper;

impl AccessTokenMapper {
    pub fn from_db(db_token: DbAccessToken) -> AccessTokenModel {
        AccessTokenModel {
            token: db_token.token,
            client_id: db_token.client_id,
            user_id: db_token.user_id,
            scopes: ScopeMapper::db_list_to_vec(&db_token.scopes),
            expires_at: db_token.expires_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};
    use uuid::Uuid;

    #[test]
    fn it_should_map_db_with_user() {
        let token = String::from("TOKEN");
        let client_id = String::from("CLIENT_ID");
        let user_id = Some(Uuid::new_v4());
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::minutes(1);
        let scopes = vec![Some(String::from("read")), Some(String::from("write"))];

        let db_token = DbAccessToken {
            id: 1,
            token: token.clone(),
            client_id: client_id.clone(),
            user_id,
            created_at,
            expires_at,
            scopes,
        };

        let actual_token = AccessTokenMapper::from_db(db_token);

        let expected_token = AccessTokenModel {
            token,
            client_id,
            user_id,
            expires_at,
            scopes: vec![String::from("read"), String::from("write")],
        };

        assert_eq!(actual_token, expected_token);
    }

    #[test]
    fn it_should_map_db_with_no_user() {
        let token = String::from("TOKEN");
        let client_id = String::from("CLIENT_ID");
        let user_id = None;
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::minutes(1);
        let scopes = vec![Some(String::from("read")), Some(String::from("write"))];

        let db_token = DbAccessToken {
            id: 1,
            token: token.clone(),
            client_id: client_id.clone(),
            user_id,
            created_at,
            expires_at,
            scopes,
        };

        let actual_token = AccessTokenMapper::from_db(db_token);

        let expected_token = AccessTokenModel {
            token,
            client_id,
            user_id,
            expires_at,
            scopes: vec![String::from("read"), String::from("write")],
        };

        assert_eq!(actual_token, expected_token);
    }
}
