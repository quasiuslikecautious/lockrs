use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use crate::db::{
    DbError,
    establish_connection,
    schema::refresh_tokens,
};

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = refresh_tokens)]
pub struct DbRefreshToken {
    pub id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub scopes: Vec<Option<String>>,
}

impl DbRefreshToken {
    pub fn get(
        token: &String,
        client_id: &String,
        user_id: &Option<Uuid>
    ) -> Result<Self, DbError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_only()
            .run(|conn| {
                refresh_tokens::table
                    .filter(refresh_tokens::token.eq(token))
                    .filter(refresh_tokens::client_id.eq(client_id))
                    .filter(refresh_tokens::created_at.lt(&now))
                    .filter(refresh_tokens::expires_at.gt(&now))
                    .filter(refresh_tokens::used.eq(false))
                    .first::<Self>(conn)
            })
            .map_err(|err| {
                match err {
                    Error::NotFound => DbError::NotFound,
                    _               => DbError::InternalError,
                }
            })
    }

    pub fn insert(
        token: &String,
        client_id: &String,
        user_id: &Option<Uuid>,
        expiry: &Duration,
        scopes: &Vec<String>
    ) -> Result<Self, DbError> {
        let expires_at = (Utc::now() + expiry.clone()).naive_utc();
        let scopes = scopes.clone().into_iter().map(|s| Some(s)).collect::<Vec<Option<String>>>();

        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(refresh_tokens::table)
                    .values((
                        refresh_tokens::token.eq(token),
                        refresh_tokens::client_id.eq(client_id),
                        refresh_tokens::user_id.eq(user_id),
                        refresh_tokens::expires_at.eq(&expires_at),
                        refresh_tokens::scopes.eq(&scopes),
                    ))
                    .get_result::<Self>(conn)
            })
            .map_err(|err| {
                match err {
                    _               => DbError::InternalError,
                }
            })
    }

    pub fn use_token(
        token: &String,
        client_id: &String,
        user_id: &Option<Uuid>
    ) -> Result<Self, DbError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_only()
            .run(|conn| {
                diesel::update(refresh_tokens::table)
                    .filter(refresh_tokens::token.eq(token))
                    .filter(refresh_tokens::client_id.eq(client_id))
                    .filter(refresh_tokens::created_at.lt(&now))
                    .filter(refresh_tokens::expires_at.gt(&now))
                    .filter(refresh_tokens::used.eq(false))
                    .set(refresh_tokens::used.eq(true))
                    .get_result::<Self>(conn)
            })
            .map_err(|err| {
                match err {
                    _               => DbError::InternalError,
                }
            })
    }
}
