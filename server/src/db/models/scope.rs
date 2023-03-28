use diesel::prelude::*;
use diesel::result::Error;

use crate::db::{
    DbError,
    establish_connection,
    schema::scopes,
};

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = scopes)]
pub struct DbScope {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub client_id: Option<String>,
}

impl DbScope {
    pub fn get(
        name: &String,
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_only()
            .run(|conn| {
                scopes::table
                    .filter(scopes::name.eq(name))
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
        name: &String,
        description: &String,
        client_id: &Option<String>
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(scopes::table)
                    .values((
                        scopes::name.eq(name),
                        scopes::description.eq(description),
                        scopes::client_id.eq(client_id),
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

