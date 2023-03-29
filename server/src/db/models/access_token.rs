use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use crate::db::{
    DbError,
    establish_connection,
    schema::access_tokens,
};

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = access_tokens)]
pub struct DbAccessToken {
    pub id: i32,
    pub token: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<Option<String>>,
}

impl DbAccessToken {
    pub fn get(
        token: &String,
        client_id: &String,
        user_id: &Option<Uuid>
    ) -> Result<Self, DbError> {
        let now = Utc::now().naive_utc();
        let connection = &mut establish_connection();

        access_tokens::table
            .filter(access_tokens::token.eq(token))
            .filter(access_tokens::client_id.eq(client_id))
            .filter(access_tokens::user_id.eq(user_id))
            .filter(access_tokens::created_at.lt(&now))
            .filter(access_tokens::expires_at.gt(&now))
            .first::<Self>(connection)
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
                diesel::insert_into(access_tokens::table)
                    .values((
                       access_tokens::token.eq(token), 
                       access_tokens::client_id.eq(client_id),
                       access_tokens::user_id.eq(user_id),
                       access_tokens::expires_at.eq(expires_at),
                       access_tokens::scopes.eq(&scopes),
                    ))
                    .get_result::<Self>(conn)
            })
            .map_err(|err| {
                match err {
                    _ => DbError::InternalError,
                }
            })

    }
}

