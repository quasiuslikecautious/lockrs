use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use url::Url;

use crate::db::{
    DbError,
    establish_connection,
    models::DbRedirectUri,
    schema::{clients, redirect_uris},
};

#[derive(Debug, Queryable, Insertable, Identifiable)]
#[diesel(primary_key(id), table_name = clients)]
pub struct DbClient {
    pub id: String,
    pub secret: Option<String>,
    pub user_id: Uuid,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: String,
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
        name: &String,
        description: &String,
        homepage_url: &Url,
        redirect_url: &Url
    ) -> Result<Self, DbError> {
        let connection = &mut establish_connection();
        connection.build_transaction()
            .read_write()
            .run(|conn| {
                let client = diesel::insert_into(clients::table)
                    .values((
                        clients::id.eq(id),
                        clients::secret.eq(secret),
                        clients::user_id.eq(user_id),
                        clients::is_public.eq(secret.is_some()),
                        clients::name.eq(name),
                        clients::description.eq(description),
                        clients::homepage_url.eq(homepage_url.to_string()),
                    ))
                    .get_result::<Self>(conn)?;

                diesel::insert_into(redirect_uris::table)
                    .values((
                        redirect_uris::client_id.eq(id),
                        redirect_uris::uri.eq(redirect_url.to_string()),
                    ))
                    .get_result::<DbRedirectUri>(conn)?;

                Ok(client)
            })
            .map_err(|err: Error| {
                match err {
                    _               => DbError::InternalError,
                }
            })
    }
}

