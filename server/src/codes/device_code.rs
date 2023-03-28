use base64::{Engine as _, engine::general_purpose};
use ring::rand::{SecureRandom, SystemRandom};
use url::Url;

use crate::{auth_response, db::models::DbDeviceCode, models};

pub struct DeviceCode {

}

impl DeviceCode {
    pub fn try_generate_code(
        client: models::Client,
        scopes: Vec<String>
    ) -> auth_response::Result<models::DeviceCodeResponse> {
        let user_code = Self::generate_user_code();
        let device_code = Self::generate_device_code();
        let expiry = chrono::Duration::minutes(10);

        DbDeviceCode::insert(
            &client.get_id(),
            &user_code,
            &device_code,
            &expiry,
            scopes
        ).map_err(|_| auth_response::Rejection::ServerError(None))?;
       
        let verification_uri = Url::parse("http://127.0.0.1:8080/device").unwrap();

        Ok(models::DeviceCodeResponse::new(
            user_code,
            device_code,
            verification_uri
        ))
    }

    pub fn poll_authorization(
        device_code: &String,
    ) -> auth_response::Result<bool> {
        todo!("implement polling handling");
    }

    fn generate_user_code() -> String {
        const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMMNOPQRSTUVWXYZ";
        const CODE_LEN: usize = 6;

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

    fn generate_device_code() -> String {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer).to_string()
    }
}
