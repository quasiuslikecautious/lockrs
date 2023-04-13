use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use url::Url;
use uuid::Uuid;

use crate::db::{self, establish_connection, schema::authorization_codes, DbError};

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = authorization_codes)]
pub struct DbAuthorizationCode {
    pub id: i32,
    pub code: String,
    pub challenge: String,
    pub is_challenge_plain: bool,
    pub client_id: String,
    pub user_id: Uuid,
    pub redirect_uri: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub scopes: Vec<Option<String>>,
}

impl DbAuthorizationCode {
    pub fn get(
        code: &String,
        challenge: &String,
        client_id: &String,
        user_id: &Uuid,
        redirect_uri: &Url,
    ) -> Result<Self, DbError> {
        let now = Utc::now().naive_utc();

        let connection = &mut db::establish_connection();
        authorization_codes::table
            .filter(authorization_codes::code.eq(code))
            .filter(authorization_codes::challenge.eq(challenge))
            .filter(authorization_codes::client_id.eq(client_id))
            .filter(authorization_codes::user_id.eq(user_id))
            .filter(authorization_codes::redirect_uri.eq(redirect_uri.to_string()))
            .filter(authorization_codes::created_at.lt(&now))
            .filter(authorization_codes::expires_at.gt(&now))
            .filter(authorization_codes::used.eq(false))
            .first::<Self>(connection)
            .map_err(|err| match err {
                Error::NotFound => DbError::NotFound,
                _ => DbError::InternalError,
            })
    }

    pub fn get_no_challenge(
        code: &String,
        client_id: &String,
        user_id: &Uuid,
    ) -> Result<Self, DbError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        authorization_codes::table
            .filter(authorization_codes::code.eq(code))
            .filter(authorization_codes::client_id.eq(client_id))
            .filter(authorization_codes::user_id.eq(user_id))
            .filter(authorization_codes::created_at.lt(&now))
            .filter(authorization_codes::expires_at.gt(&now))
            .filter(authorization_codes::used.eq(false))
            .first::<Self>(connection)
            .map_err(|err| match err {
                Error::NotFound => DbError::NotFound,
                _ => DbError::InternalError,
            })
    }

    pub fn insert(
        code: &String,
        challenge: &String,
        is_challenge_plain: &bool,
        client_id: &String,
        user_id: &Uuid,
        redirect_uri: &Url,
        expiry: &Duration,
        scopes: Vec<String>,
    ) -> Result<Self, DbError> {
        let expires_at = (Utc::now() + *expiry).naive_utc();
        let scopes = scopes
            .into_iter()
            .map(Some)
            .collect::<Vec<Option<String>>>();

        let connection = &mut establish_connection();
        connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(authorization_codes::table)
                    .values((
                        authorization_codes::code.eq(code),
                        authorization_codes::challenge.eq(challenge),
                        authorization_codes::is_challenge_plain.eq(is_challenge_plain),
                        authorization_codes::client_id.eq(client_id),
                        authorization_codes::user_id.eq(user_id),
                        authorization_codes::redirect_uri.eq(redirect_uri.to_string()),
                        authorization_codes::expires_at.eq(expires_at),
                        authorization_codes::scopes.eq(&scopes),
                    ))
                    .get_result::<Self>(conn)
            })
            .map_err(|_err| DbError::InternalError)
    }

    pub fn use_token(&self) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                diesel::update(authorization_codes::table)
                    .filter(authorization_codes::id.eq(self.id))
                    .set(authorization_codes::used.eq(false))
                    .get_result::<Self>(conn)
            })
            .map_err(|_err| DbError::InternalError)
    }
}
