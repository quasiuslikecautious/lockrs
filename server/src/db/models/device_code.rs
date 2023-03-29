use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;

use crate::db::{
    DbError,
    establish_connection,
    schema::device_codes,
};

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = device_codes)]
pub struct DbDeviceCode {
   pub id: i32,
   pub client_id: String,
   pub user_code: String,
   pub device_code: String,
   pub created_at: NaiveDateTime,
   pub expires_at: NaiveDateTime,
   pub scopes: Vec<Option<String>>,
}

impl DbDeviceCode {
    pub fn get_from_device_code(
        client_id: &String,
        device_code: &String,
    ) -> Result<Self, DbError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        device_codes::table
            .filter(device_codes::client_id.eq(client_id))
            .filter(device_codes::device_code.eq(device_code))
            .filter(device_codes::created_at.lt(now))
            .filter(device_codes::expires_at.gt(now))
            .first::<Self>(connection)
        .map_err(|err| {
            match err {
                Error::NotFound => DbError::NotFound,
                _               => DbError::InternalError,
            }
        })
    }
    
    pub fn get_from_user_code(
        client_id: &String,
        user_code: &String,
    ) -> Result<Self, DbError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        device_codes::table
            .filter(device_codes::client_id.eq(client_id))
            .filter(device_codes::user_code.eq(user_code))
            .filter(device_codes::created_at.lt(now))
            .filter(device_codes::expires_at.gt(now))
            .first::<Self>(connection)
        .map_err(|err| {
            match err {
                Error::NotFound => DbError::NotFound,
                _               => DbError::InternalError,
            }
        })
    }

    pub fn insert(
        client_id: &String,
        user_code: &String,
        device_code: &String,
        expiry: &Duration,
        scopes: Vec<String>,
    ) -> Result<Self, DbError> {
        let expires_at = (Utc::now() + expiry.clone()).naive_utc();
        let scopes = scopes.clone().into_iter().map(|s| Some(s)).collect::<Vec<Option<String>>>();

        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(device_codes::table)
                    .values((
                        device_codes::client_id.eq(client_id),
                        device_codes::user_code.eq(user_code),
                        device_codes::device_code.eq(device_code),
                        device_codes::expires_at.eq(expires_at),
                        device_codes::scopes.eq(scopes),
                    ))
                    .get_result::<Self>(conn)
            })
            .map_err(|err| {
                match err {
                    _               => DbError::InternalError,
                }
            })
    }
}

