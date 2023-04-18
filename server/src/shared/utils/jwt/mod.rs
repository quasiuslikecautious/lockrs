mod jwt_claims;
mod rotating_key;

use std::sync::Arc;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use self::{jwt_claims::*, rotating_key::*};

pub struct JwtUtil {
    secret: Arc<RotatingKey>,
}

impl JwtUtil {
    pub fn sign_jwt<T>(&self, claims: T) -> Result<String, jsonwebtoken::errors::Error>
    where
        T: Serialize,
    {
        let secret_key = &self.secret.get_signing_key();

        let header = Header::new(Algorithm::HS256);
        let jwt_claims = JwtClaims {
            claims,
            version: secret_key.as_ref().version,
        };

        encode(
            &header,
            &jwt_claims,
            &EncodingKey::from_secret(&secret_key.as_ref().value[..]),
        )
    }

    pub fn verify_jwt<T>(&self, token: &str) -> Result<JwtClaims<T>, jsonwebtoken::errors::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let secret_key = &self.secret.get_signing_key();

        let token = decode::<JwtClaims<T>>(
            token,
            &DecodingKey::from_secret(&secret_key.as_ref().value[..]),
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(token.claims)
    }
}
