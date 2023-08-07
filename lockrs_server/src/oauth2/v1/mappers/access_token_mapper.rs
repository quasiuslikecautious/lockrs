use crate::{db::pg::models::PgAccessToken, oauth2::v1::models::AccessTokenModel};

use super::ScopeMapper;

pub struct AccessTokenMapper;

impl AccessTokenMapper {
    pub fn from_pg(pg_token: PgAccessToken) -> AccessTokenModel {
        AccessTokenModel::new(
            pg_token.id,
            pg_token.token.as_str(),
            pg_token.client_id.as_str(),
            pg_token.user_id.as_ref(),
            &pg_token.expires_at,
            ScopeMapper::pg_list_to_vec(&pg_token.scopes).as_slice(),
        )
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
        let token = String::from("TOKEN");
        let client_id = String::from("CLIENT_ID");
        let user_id = Some(Uuid::new_v4());
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::minutes(1);
        let scopes = vec![Some(String::from("read")), Some(String::from("write"))];

        let pg_token = PgAccessToken {
            id,
            token: token.clone(),
            client_id: client_id.clone(),
            user_id,
            created_at,
            expires_at,
            scopes,
        };

        let actual_token = AccessTokenMapper::from_pg(pg_token);

        let expected_token = AccessTokenModel::new(
            id,
            token.as_str(),
            client_id.as_str(),
            user_id.as_ref(),
            &expires_at,
            &[String::from("read"), String::from("write")],
        );

        assert_eq!(actual_token, expected_token);
    }

    #[test]
    fn it_should_map_pg_with_no_user() {
        let id = 1;
        let token = String::from("TOKEN");
        let client_id = String::from("CLIENT_ID");
        let user_id = None;
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::minutes(1);
        let scopes = vec![Some(String::from("read")), Some(String::from("write"))];

        let pg_token = PgAccessToken {
            id,
            token: token.clone(),
            client_id: client_id.clone(),
            user_id,
            created_at,
            expires_at,
            scopes,
        };

        let actual_token = AccessTokenMapper::from_pg(pg_token);

        let expected_token = AccessTokenModel::new(
            id,
            token.as_str(),
            client_id.as_str(),
            user_id.as_ref(),
            &expires_at,
            &[String::from("read"), String::from("write")],
        );

        assert_eq!(actual_token, expected_token);
    }
}
