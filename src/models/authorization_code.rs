use base64::{Engine as _, engine::general_purpose};
use diesel::prelude::*;
use ring::{
    digest::{digest, SHA256},
    rand::{SecureRandom, SystemRandom},
};
use url::Url;

use crate::{auth_response, db, models, schema};

/// The authorization grant code supplied in the authorization grant step of the auth flow
pub struct AuthorizationCode {
    client: models::Client,
    user: models::User,
}

impl AuthorizationCode {
    pub fn new(
        client: &models::Client, 
        user: &models::User
    )-> Self {
        Self {
            client: client.clone(),
            user: user.clone(),
      }
    }

    pub fn try_generate(&self, challenge: &str, is_plain: bool, redirect: &Url, scopes: Vec<String>) -> auth_response::Result<String> {
        use schema::authorization_codes;

        let code = Self::generate_code();
        let expiry = (chrono::Utc::now() + chrono::Duration::minutes(5)).naive_utc();

        let connection = &mut db::establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(authorization_codes::table)
                    .values((
                        authorization_codes::code.eq(&code),
                        authorization_codes::challenge.eq(challenge),
                        authorization_codes::is_challenge_plain.eq(is_plain),
                        authorization_codes::client_id.eq(&self.client.get_id()),
                        authorization_codes::user_id.eq(&self.user.get_id()),
                        authorization_codes::redirect_uri.eq(redirect.as_str()),
                        authorization_codes::expires_at.eq(&expiry),
                    ))
                    .execute(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(None))?;

        Ok(code)
    }

    fn generate_code() -> String {
        const CODE_LEN: usize = 32;

        let mut buffer = [0u8; CODE_LEN];
        let rng = SystemRandom::new();

        rng.fill(&mut buffer).unwrap();

        let code = general_purpose::URL_SAFE_NO_PAD.encode(buffer);

        code
    }

    /// Decrypt/Verify (and remove from db if necessary) provided code
    pub fn validate(&self, code: &str, verifier: &str, redirect: &Url) -> auth_response::Result<()> {
        use schema::authorization_codes;

        let now = chrono::Utc::now().naive_utc();
        
        let connection = &mut db::establish_connection();
        let db_code = connection.build_transaction()
            .read_only()
            .run(|conn| {
                authorization_codes::table
                    .filter(authorization_codes::code.eq(code))
                    .filter(authorization_codes::client_id.eq(&self.client.get_id()))
                    .filter(authorization_codes::user_id.eq(&self.user.get_id()))
                    .filter(authorization_codes::redirect_uri.eq(&redirect.as_str()))
                    .filter(authorization_codes::expires_at.gt(&now))
                    .filter(authorization_codes::used.eq(false))
                    .first::<db::DbAuthorizationCode>(conn)
            })
            .map_err(|_| auth_response::Rejection::InvalidGrant)?;

        let expected = match &db_code.is_challenge_plain {
            true => verifier.to_string(),
            false => {
                let byte_verifier = verifier.as_bytes();
                
                // for some reason using .as_ref() on a ring::Digest object produces the incorrect
                // byte array with invalid utf-8 characters, so instead we must use the following
                // "suboptimal" way to extract correct value from sha256 digest.
                let sha_digest = digest(&SHA256, byte_verifier);
                let sha_str = format!("{:?}", &sha_digest);
                let sha_encoded = sha_str.split_once(':').unwrap().1;
                let sha_bytes = sha_encoded.as_bytes();

                let base_encoded = general_purpose::URL_SAFE_NO_PAD.encode(sha_bytes);
                
                base_encoded.clone()
            }
        };

        if &expected != &db_code.challenge {
            return Err(auth_response::Rejection::InvalidRequest);
        }

        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::update(authorization_codes::table)
                    .filter(authorization_codes::id.eq(&db_code.id))
                    .set(authorization_codes::used.eq(true))
                    .execute(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(None))?;
        
        Ok(())
    }
}
