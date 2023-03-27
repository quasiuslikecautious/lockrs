use base64::{Engine as _, engine::general_purpose};
use diesel::prelude::*;
use ring::rand::{SecureRandom, SystemRandom};
use url::Url;
use uuid::Uuid;

use crate::{
    auth_response,
    db, 
    models,
    schema,
};

pub struct AuthToken {
    pub access: String,
    pub refresh: String,
    pub scopes: String,
}

#[derive(Debug)]
pub struct TokenBuilder {
    client: models::Client,
    user: Option<models::User>,
    scopes: models::Scopes,
    redirect_uri: Option<Url>,
}

impl TokenBuilder {
    pub fn new(
        client: models::Client,
        user: Option<models::User>,
        scopes: models::Scopes,
        redirect_uri: Option<Url>,
    ) -> Self {
        Self {
            client,
            user,
            scopes,
            redirect_uri,
        }
    }

    pub fn try_build(&self) -> auth_response::Result<AuthToken> {
        Ok(AuthToken {
            access: self.generate_access_token()?,
            refresh: self.generate_refresh_token()?,
            scopes: self.scopes.into_scope_string(),
        })
    } 

    pub fn validate_access_token(
        client: &models::Client,
        user: &Option<models::User>,
        token: &str
    ) -> auth_response::Result<models::Scopes> {
        use schema::access_tokens;

        let user_id = user.clone().map(|u| u.get_id());
        let now = chrono::Utc::now().naive_utc();

        let connection = &mut db::establish_connection();
        let db_token = connection.build_transaction()
            .read_only()
            .run(|conn| {
                access_tokens::table
                    .filter(access_tokens::token.eq(token))
                    .filter(access_tokens::client_id.eq(client.get_id()))
                    .filter(access_tokens::user_id.eq(&user_id))
                    .filter(access_tokens::expires_at.gt(&now))
                    .first::<db::DbAccessToken>(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(None))?;

        let scopes = db_token.scopes
            .into_iter()
            .filter_map(|s| s)
            .collect::<Vec<String>>();

        Ok(models::Scopes::new(scopes))
    }

    pub fn validate_refresh_token(
        client: &models::Client,
        token: &str
    ) -> auth_response::Result<models::Scopes> {
        use schema::refresh_tokens;

        let now = chrono::Utc::now().naive_utc();

        let connection = &mut db::establish_connection();
        let db_token = connection.build_transaction()
            .read_only()
            .run(|conn| {
                refresh_tokens::table
                    .filter(refresh_tokens::token.eq(token))
                    .filter(refresh_tokens::client_id.eq(client.get_id()))
                    .filter(refresh_tokens::expires_at.gt(&now))
                    .filter(refresh_tokens::used.eq(false))
                    .first::<db::DbRefreshToken>(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(None))?;

        let scopes = db_token.scopes
            .into_iter()
            .filter_map(|s| s)
            .collect::<Vec<String>>();

        Ok(models::Scopes::new(scopes))
    }

    fn generate_token() -> String {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer).to_string()
    }

    fn generate_access_token(&self) -> auth_response::Result<String> {
        use schema::access_tokens;

        let user_id = self.user.clone().map(|u| u.get_id());

        let new_token = Self::generate_token();
        let expiry = (chrono::Utc::now() + chrono::Duration::minutes(10)).naive_utc();
        let connection = &mut db::establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(access_tokens::table)
                    .values((
                        access_tokens::token.eq(&new_token),
                        access_tokens::client_id.eq(&self.client.get_id()),
                        access_tokens::user_id.eq(&user_id),
                        access_tokens::expires_at.eq(expiry),
                        access_tokens::scopes.eq(&self.scopes.get()),
                    ))
                    .execute(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(self.redirect_uri.clone()))?;

        Ok(new_token)
    }

    fn generate_refresh_token(&self) -> auth_response::Result<String> {
        use schema::refresh_tokens;

        let user_id = self.user.clone().map(|u| u.get_id());

        let new_token = Self::generate_token();
        let expiry = (chrono::Utc::now() + chrono::Duration::hours(24)).naive_utc();
        let connection = &mut db::establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(refresh_tokens::table)
                    .values((
                        refresh_tokens::token.eq(&new_token),
                        refresh_tokens::client_id.eq(&self.client.get_id()),
                        refresh_tokens::user_id.eq(&user_id),
                        refresh_tokens::expires_at.eq(&expiry),
                        refresh_tokens::scopes.eq(&self.scopes.get()),
                    ))
                    .execute(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(self.redirect_uri.clone()))?;

        Ok(new_token)
    }
}

