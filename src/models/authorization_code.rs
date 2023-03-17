use diesel::prelude::*;
use ring::rand::{SecureRandom, SystemRandom};
use url::Url;

use crate::{auth_response, db, models, schema};

/// The authorization grant code supplied in the authorization grant step of the auth flow
pub struct AuthorizationCode {
    client: models::ValidatedClient,
    user: models::ValidatedUser,
}

impl AuthorizationCode {
    pub fn new(
        client: &models::ValidatedClient, 
        user: &models::ValidatedUser
    )-> Self {
        Self {
            client: client.clone(),
            user: user.clone(),
      }
    }

    pub fn try_generate(&self, redirect: &Url, scopes: Vec<String>) -> auth_response::Result<String> {
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
// juQdDmb_bgHttGEQ-TBoJzyvcZ62AYo1
    fn generate_code() -> String {
        const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_.";
        const CODE_LEN: usize = 32;

        let mut code = String::with_capacity(CODE_LEN);
        let mut buffer = [0u8; CODE_LEN];
        let rng = SystemRandom::new();

        rng.fill(&mut buffer).unwrap();

        for byte in buffer.iter() {
            let idx = byte % ALPHABET.len() as u8;
            let c = char::from(ALPHABET[idx as usize]);
            code.push(c);
        }

        code
    }

    /// Decrypt/Verify (and remove from db if necessary) provided code
    pub fn validate(&self, code: &String, redirect: &Url) -> auth_response::Result<()> {
        use schema::authorization_codes;

        let now = chrono::Utc::now().naive_utc();

        let connection = &mut db::establish_connection();
        let affected_lines = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::update(authorization_codes::table)
                    .filter(authorization_codes::code.eq(code))
                    .filter(authorization_codes::client_id.eq(&self.client.get_id()))
                    .filter(authorization_codes::user_id.eq(&self.user.get_id()))
                    .filter(authorization_codes::redirect_uri.eq(&redirect.as_str()))
                    .filter(authorization_codes::expires_at.gt(&now))
                    .filter(authorization_codes::used.eq(false))
                    .set(authorization_codes::used.eq(true))
                    .execute(conn)
            })
            .map_err(|_| auth_response::Rejection::InvalidGrant)?;

        if affected_lines != 1 {
            return Err(auth_response::Rejection::InvalidGrant);
        }

        Ok(())
    }
}
