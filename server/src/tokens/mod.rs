pub mod token_error;

use base64::{Engine as _, engine::general_purpose};
use ring::rand::{SecureRandom, SystemRandom};
use url::Url;

use crate::{
    auth_response,
    db::{DbError, models::{DbAccessToken, DbRefreshToken}}, 
    models,
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
        token: &String
    ) -> auth_response::Result<models::Scopes> {
        let user_id = user.clone().map(|u| u.get_id());
        let db_token = DbAccessToken::get(token, &client.get_id(), &user_id)
            .map_err(|_| auth_response::Rejection::ServerError(None))?;

        let scopes = db_token.scopes
            .into_iter()
            .filter_map(|s| s)
            .collect::<Vec<String>>();

        Ok(models::Scopes::new(scopes))
    }

    pub fn validate_refresh_token(
        client: &models::Client,
        user: &Option<models::User>,
        token: &String
    ) -> auth_response::Result<models::Scopes> {
        let user_id = user.clone().map(|u| u.get_id());
        let db_token = DbRefreshToken::get(token, &client.get_id(), &user_id)
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
        let new_token = Self::generate_token();
        let user_id = self.user.clone().map(|u| u.get_id());
        let expiry = chrono::Duration::minutes(10);

        DbAccessToken::insert(&new_token, &self.client.get_id(), &user_id, &expiry, &self.scopes.get())
            .map_err(|_| auth_response::Rejection::ServerError(self.redirect_uri.clone()))?;

        Ok(new_token)
    }

    fn generate_refresh_token(&self) -> auth_response::Result<String> {
        let new_token = Self::generate_token();
        let user_id = self.user.clone().map(|u| u.get_id());
        let expiry = chrono::Duration::hours(24);

        DbRefreshToken::insert(&new_token, &self.client.get_id(), &user_id, &expiry, &self.scopes.get())
            .map_err(|_| auth_response::Rejection::ServerError(self.redirect_uri.clone()))?;

        Ok(new_token)
    }
}

