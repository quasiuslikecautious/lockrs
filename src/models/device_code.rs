use base64::{Engine as _, engine::general_purpose};
use diesel::prelude::*;
use ring::rand::{SecureRandom, SystemRandom};
use url::Url;

use crate::{auth_response, db, models::{self, response}, schema};



pub struct DeviceCode {
    pub client: models::ValidatedClient,
    pub scopes: Vec<String>,
}

impl DeviceCode {
    pub fn new(client: models::ValidatedClient, scopes: Vec<String>) -> Self {
        Self {
            client,
            scopes,
        }
    }
    
    pub fn try_generate_code(&self) -> auth_response::Result<response::DeviceCodeResponse> {
        use schema::device_codes;

        let expiry = (chrono::Utc::now() + chrono::Duration::minutes(10)).naive_utc();
        let user_code = Self::generate_user_code();
        let device_code = Self::generate_device_code();


        let connection = &mut db::establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(device_codes::table)
                    .values((
                        device_codes::client_id.eq(&self.client.get_id()),
                        device_codes::user_code.eq(&user_code),
                        device_codes::device_code.eq(&device_code),
                        device_codes::expires_at.eq(&expiry),
                        device_codes::scopes.eq(&self.scopes),
                    ))
                    .execute(conn)
            })
            .map_err(|_| auth_response::Rejection::ServerError(None))?;

        let verification_uri = Url::parse("http://127.0.0.1:8080/device").unwrap();

        Ok(response::DeviceCodeResponse::new(
            user_code,
            device_code,
            verification_uri,
        ))
    }

    pub fn poll_authorization(
        device_code: &String,
    ) -> auth_response::Result<bool> {
        return Ok(true);
    }

    fn generate_user_code() -> String {
        const CHARSET: &[u8] = b"0123456789BCDFGHJKLMMNPQRSTVWXZ";
        const CODE_LEN: usize = 8;

        let mut code = String::with_capacity(CODE_LEN);
        let mut buffer = [0u8; CODE_LEN];
        let rng = SystemRandom::new();

        rng.fill(&mut buffer).unwrap();
        
        for byte in buffer.iter() {
            let idx = byte % CHARSET.len() as u8;
            let c = char::from(CHARSET[idx as usize]);
            code.push(c);
        }
        
        code
    }

    fn generate_device_code() -> String {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer).to_string()
    }
}
