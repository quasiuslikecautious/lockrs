use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use crate::db::{
    DbError,
    establish_connection,
    schema::clients,
};

#[derive(Debug, Queryable, Insertable, Identifiable)]
#[diesel(primary_key(id), table_name = clients)]
pub struct DbClient {
    pub id: String,
    pub secret: Option<String>,
    pub user_id: Uuid,
    pub is_public: bool,
    pub name: String,
}

impl DbClient {
    pub fn get(
        id: &String,
        secret: &Option<String>
    ) -> Result<Self, DbError> {
        let mut query = clients::table
            .into_boxed()
            .filter(clients::id.eq(&id));

        if let Some(client_secret) = secret {
            query = query.filter(clients::secret.eq(client_secret));
        }
        
        let connection = &mut establish_connection();
        query.first::<Self>(connection).map_err(|err| {
            match err {
                Error::NotFound => DbError::NotFound,
                _               => DbError::InternalError,
            }
        })
    }
    
    pub fn insert(
        id: &String,
        secret: &Option<String>,
        user_id: &Uuid,
        name: &String
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(clients::table)
                    .values((
                        clients::id.eq(id),
                        clients::secret.eq(secret),
                        clients::user_id.eq(user_id),
                        clients::is_public.eq(secret.is_some()),
                        clients::name.eq(name),
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

