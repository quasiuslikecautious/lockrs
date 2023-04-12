use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use ring::rand::{SecureRandom, SystemRandom};
use uuid::Uuid;

use crate::{
    db::{
        establish_connection,
        models::{DbAccessToken, DbRefreshToken},
        schema::{access_tokens, refresh_tokens},
    },
    oauth2::{
        mappers::RefreshTokenMapper,
        models::{RefreshTokenModel, ScopesModel},
        responses::TokenResponse,
    },
};

pub struct TokenService;

impl TokenService {
    pub fn create_token(
        client_id: &str,
        user_id: &Option<Uuid>,
        scopes: ScopesModel,
    ) -> Result<TokenResponse, TokenServiceError> {
        let access_expiry = (Utc::now() + Duration::minutes(10)).naive_utc();
        let refresh_expiry = (Utc::now() + Duration::hours(24)).naive_utc();

        let connection = &mut establish_connection();
        let (access_token, refresh_token) = connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                let access_token = diesel::insert_into(access_tokens::table)
                    .values((
                        access_tokens::token.eq(Self::generate_opaque_token()),
                        access_tokens::client_id.eq(&client_id),
                        access_tokens::user_id.eq(user_id),
                        access_tokens::expires_at.eq(access_expiry),
                        access_tokens::scopes.eq(&scopes.scopes),
                    ))
                    .get_result::<DbAccessToken>(conn)?;

                let refresh_token = diesel::insert_into(refresh_tokens::table)
                    .values((
                        refresh_tokens::token.eq(Self::generate_opaque_token()),
                        refresh_tokens::client_id.eq(&client_id),
                        refresh_tokens::user_id.eq(user_id),
                        refresh_tokens::expires_at.eq(refresh_expiry),
                        refresh_tokens::scopes.eq(&scopes.scopes),
                    ))
                    .get_result::<DbRefreshToken>(conn)?;

                Ok((access_token, refresh_token))
            })
            .map_err(|err: diesel::result::Error| TokenServiceError::from(err))?;

        Ok(TokenResponse {
            token_type: String::from("Bearer"),
            expires_in: 5000,
            access_token: access_token.token,
            refresh_token: refresh_token.token,
            scopes: scopes
                .scopes
                .into_iter()
                .fold(String::new(), |c, s| format!("{} {}", c, s)),
        })
    }

    pub fn verify_access_token(
        client_id: &str,
        user_id: &Option<Uuid>,
        token: &str,
    ) -> Result<ScopesModel, TokenServiceError> {
        let now = Utc::now().naive_utc();
        let connection = &mut establish_connection();

        let db_token = access_tokens::table
            .filter(access_tokens::token.eq(token))
            .filter(access_tokens::client_id.eq(client_id))
            .filter(access_tokens::user_id.eq(user_id))
            .filter(access_tokens::created_at.lt(&now))
            .filter(access_tokens::expires_at.gt(&now))
            .first::<DbAccessToken>(connection)
            .map_err(|err| TokenServiceError::from(err))?;

        let scopes = db_token
            .scopes
            .into_iter()
            .filter_map(|s| match s {
                Some(s) => Some(s),
                None => None,
            })
            .collect::<Vec<String>>();

        Ok(ScopesModel { scopes })
    }

    pub fn verify_refresh_token(
        client_id: &str,
        token: &str,
    ) -> Result<RefreshTokenModel, TokenServiceError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        let db_token = refresh_tokens::table
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::client_id.eq(client_id))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .first::<DbRefreshToken>(connection)
            .map_err(|err| TokenServiceError::from(err))?;

        Ok(RefreshTokenMapper::from_db(db_token))
    }

    pub fn use_refresh_token(client_id: &str, token: &str) -> Result<(), TokenServiceError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                diesel::update(refresh_tokens::table)
                    .filter(refresh_tokens::token.eq(token))
                    .filter(refresh_tokens::client_id.eq(client_id))
                    .filter(refresh_tokens::created_at.lt(&now))
                    .filter(refresh_tokens::expires_at.gt(&now))
                    .filter(refresh_tokens::used.eq(false))
                    .set(refresh_tokens::used.eq(true))
                    .get_result::<DbRefreshToken>(conn)
            })
            .map_err(|err| TokenServiceError::from(err))?;

        Ok(())
    }

    pub fn generate_opaque_token() -> String {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer).to_string()
    }
}

pub enum TokenServiceError {
    DbError,
    NotFound,
}

impl From<diesel::result::Error> for TokenServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::NotFound,
            _ => Self::DbError,
        }
    }
}
