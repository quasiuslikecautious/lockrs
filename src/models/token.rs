use base64::{Engine as _, engine::general_purpose};
use diesel::prelude::*;
use ring::rand::{SecureRandom, SystemRandom};
use url::Url;

use crate::{
    auth_response,
    db, 
    models::{
        ValidatedClient, 
        ValidatedUser
    },
    schema,
};

pub struct AuthToken {
    pub access: String,
    pub refresh: String,
    pub scopes: String,
}

#[derive(Debug)]
pub struct TokenBuilder {
    client: ValidatedClient ,
    user: Option<ValidatedUser>,
    scopes: String,
    redirect_uri: Option<Url>,
}

impl TokenBuilder {
    pub fn new(
        client: ValidatedClient,
        user: Option<ValidatedUser>,
        scopes: String,
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
            scopes: self.scopes.clone(),
        })
    } 

    fn generate_token() -> String {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer).to_string()
    }

    fn generate_access_token(&self) -> auth_response::Result<String> {
        use schema::access_tokens::dsl::*;

        let new_token = Self::generate_token();
        let expiry = (chrono::Utc::now() + chrono::Duration::minutes(10)).naive_utc();
        let parsed_scopes = &self.scopes.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
        let mapped_user_id = match &self.user {
            Some(u) => Some(u.get_id()),
            None => None,
        };

        let connection = &mut db::establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(access_tokens)
                    .values((
                        token.eq(&new_token),
                        client_id.eq(&self.client.get_id()),
                        user_id.eq(mapped_user_id),
                        expires_at.eq(expiry),
                        scopes.eq(&parsed_scopes),
                    ))
                    .execute(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(self.redirect_uri.clone()))?;

        Ok(new_token)
    }

    fn generate_refresh_token(&self) -> auth_response::Result<String> {
        use schema::refresh_tokens::dsl::*;
        let new_token = Self::generate_token();
        let expiry = (chrono::Utc::now() + chrono::Duration::hours(24)).naive_utc();
        let parsed_scopes = &self.scopes.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
        let mapped_user_id = match &self.user {
            Some(u) => Some(u.get_id()),
            None => None,
        };
        let connection = &mut db::establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(refresh_tokens)
                    .values((
                        token.eq(&new_token),
                        client_id.eq(&self.client.get_id()),
                        user_id.eq(&mapped_user_id),
                        expires_at.eq(&expiry),
                        scopes.eq(&parsed_scopes),
                    ))
                    .execute(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(self.redirect_uri.clone()))?;

        Ok(new_token)
    }
}

