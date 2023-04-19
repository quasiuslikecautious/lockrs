mod jwt_claims;
mod key_version_extractor;
mod rotating_key;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use self::key_version_extractor::extract_key_version_from_token;
pub use self::{jwt_claims::*, rotating_key::*};

pub struct JwtUtil {
    pub secret: RotatingKey,
}

impl JwtUtil {
    pub fn sign_jwt<T>(&self, claims: T) -> Result<String, JwtError>
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
        .map_err(|_| JwtError::CreateToken)
    }

    pub fn verify_jwt<T>(&self, token: &str) -> Result<JwtClaims<T>, JwtError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let key_version =
            extract_key_version_from_token(token).ok_or(JwtError::MissingKeyVersion)?;
        let secret_key = &self
            .secret
            .get_verification_key(key_version)
            .ok_or(JwtError::Secret)?;

        let token = decode::<JwtClaims<T>>(
            token,
            &DecodingKey::from_secret(&secret_key.as_ref().value[..]),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| JwtError::InvalidToken)?;

        Ok(token.claims)
    }
}

#[derive(Debug)]
pub enum JwtError {
    InvalidToken,
    Secret,
    CreateToken,
    MissingKeyVersion,
}
