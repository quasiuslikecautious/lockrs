use diesel::prelude::*;

use crate::{
    db::{establish_connection, schema::scopes}, 
    oauth2::models::Scopes
};

pub struct ScopeService;

impl ScopeService {
    pub fn get_from_list(scope: &str) -> Result<Scopes, ScopeServiceError> {
        let scopes_list = scope.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
    
        let connection = &mut establish_connection();
        let validated_scopes = connection.build_transaction()
            .read_only()
            .run(|conn| {
                scopes::table
                    .select(scopes::name)
                    .filter(scopes::name.eq_any(&scopes_list))
                    .load(conn)
            })
            .map_err(|err| {
                match err {
                    diesel::result::Error::NotFound => ScopeServiceError::InvalidScopes,
                    _ => ScopeServiceError::DbError,
                }
            })?;

        return Ok(Scopes {
            scopes: validated_scopes,
        });       
    }
}

pub enum ScopeServiceError {
    DbError,
    InvalidScopes,
}

impl From<diesel::result::Error> for ScopeServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::InvalidScopes,
            _ => Self::DbError,
        }
    }
}


