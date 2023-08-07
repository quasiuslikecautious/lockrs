use url::Url;

use crate::{db::pg::models::PgRedirectUri, models::RedirectModel};

pub struct RedirectMapper;

impl RedirectMapper {
    pub fn from_pg(pg_redirect: PgRedirectUri) -> RedirectModel {
        RedirectModel::new(
            &pg_redirect.id,
            pg_redirect.client_id.as_str(),
            &Url::parse(&pg_redirect.uri)
                .unwrap_or_else(|_| panic!("invalid url stored in database: {}", pg_redirect.id)),
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn it_should_map_pg() {
        let id = 1;
        let client_id = String::from("CLIENT_ID");
        let uri = Url::parse("https://127.0.0.1/oauth2/callback").unwrap();
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let pg_redirect = PgRedirectUri {
            id,
            client_id: client_id.clone(),
            uri: uri.to_string(),
            created_at: created_at.naive_utc(),
            updated_at: updated_at.naive_utc(),
        };

        let actual_redirect = RedirectMapper::from_pg(pg_redirect);

        let expected_redirect = RedirectModel::new(&id, client_id.as_str(), &uri);

        assert_eq!(actual_redirect, expected_redirect);
    }
}
