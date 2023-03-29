use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use url::Url;

use crate::db::{
    DbError,
    establish_connection,
    models::DbClient,
    schema::{redirect_uris},
};

#[derive(Debug, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(DbClient, foreign_key = client_id))]
#[diesel(primary_key(id), table_name = redirect_uris)]
pub struct DbRedirectUri {
    pub id: i32,
    pub client_id: String,
    pub uri: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl DbRedirectUri {
    pub fn get(
        client_id: &String,
        uri: &Url
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .filter(redirect_uris::uri.eq(uri.to_string()))
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
        uri: &Url
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(redirect_uris::table)
                    .values((
                        redirect_uris::client_id.eq(client_id),
                        redirect_uris::uri.eq(uri.to_string())
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

