use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use url::Url;

use crate::db::{
    DbError,
    establish_connection,
    schema::{clients, redirect_uris},
};

#[derive(Debug, Queryable, Insertable)]
#[diesel(primary_key(id), table_name = clients)]
pub struct DbClient {
    pub id: String,
    pub secret: Option<String>,
    pub user_id: Uuid,
    pub redirect_uri: String,
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
        redirect_uri: &Url,
        name: &String
    ) -> Result<Self, DbError> {
        let redirect_uri = redirect_uri.to_string();

        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(redirect_uris::table)
                    .values((
                        redirect_uris::client_id.eq(id),
                        redirect_uris::uri.eq(&redirect_uri),
                    ))
                    .execute(conn)?;

                diesel::insert_into(clients::table)
                    .values((
                        clients::id.eq(id),
                        clients::secret.eq(secret),
                        clients::user_id.eq(user_id),
                        clients::redirect_uri.eq(&redirect_uri),
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

