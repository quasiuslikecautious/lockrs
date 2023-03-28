use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use crate::db::{
    DbError,
    establish_connection,
    schema::users,
};

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = users)]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

impl DbUser {
    pub fn get_from_id(
        id: &Uuid
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_only()
            .run(|conn| {
                users::table
                    .filter(users::id.eq(id))
                    .first::<Self>(conn)
            })
            .map_err(|err| {
                match err {
                    Error::NotFound => DbError::NotFound,
                    _               => DbError::InternalError,
                }
            })
    }

    pub fn get_from_credentials(
        email: &String,
        password_hash: &String
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_only()
            .run(|conn| {
                users::table
                    .filter(users::email.eq(email))
                    .filter(users::password_hash.eq(password_hash))
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
        email: &String,
        password_hash: &String
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(users::table)
                    .values((
                        users::email.eq(email),
                        users::password_hash.eq(password_hash)
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

