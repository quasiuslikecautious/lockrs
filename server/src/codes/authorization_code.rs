use base64::{Engine as _, engine::general_purpose};
use ring::{
    digest::{digest, SHA256},
    rand::{SecureRandom, SystemRandom},
};
use url::Url;
use uuid::uuid;

use crate::{
    auth_response, 
    db::{models::DbAuthorizationCode, DbError}, 
    models,
};

/// The authorization grant code supplied in the authorization grant step of the auth flow
pub struct AuthorizationCode {
    
}

impl AuthorizationCode {
    pub fn try_generate(
        client: &models::Client, 
        challenge: &String, 
        is_plain: bool, 
        redirect: &Url, 
        scopes: Vec<String>
    ) -> auth_response::Result<String> {
        let code = Self::generate_code();
        let expiry = chrono::Duration::minutes(5);

        let user_id = uuid!("00000000-0000-0000-0000-000000000000");

        DbAuthorizationCode::insert(
            &code,
            &challenge,
            &is_plain,
            &client.get_id(),
            &user_id,
            &redirect,
            &expiry,
            scopes
        ).map_err(|_| auth_response::Rejection::ServerError(None))?;

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
    pub fn validate(
        client: &models::Client,
        code: &String,
        verifier: &String,
        redirect: &Url
    ) -> auth_response::Result<models::Scopes> {
        let user_id = uuid!("00000000-0000-0000-0000-000000000000");

        let db_code = DbAuthorizationCode::get_no_challenge(
            code,
            &client.get_id(),
            &user_id
        )
        .map_err(|err| {
            match err {
                DbError::NotFound => auth_response::Rejection::InvalidGrant,
                DbError::InternalError => auth_response::Rejection::ServerError(None),
            }
        })?;

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

        let db_code = db_code
            .use_token()
            .map_err(|_| auth_response::Rejection::ServerError(None))?;
        
        let scopes = db_code.scopes
            .into_iter()
            .filter_map(|s| s)
            .collect::<Vec<String>>();
        
        Ok(models::Scopes::new(scopes))
    }
}
