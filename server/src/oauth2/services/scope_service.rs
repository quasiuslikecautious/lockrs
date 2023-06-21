use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{oauth2::models::ScopesModel, pg::schema::scopes};

pub struct ScopeService;

impl ScopeService {
    pub async fn get_from_list(
        connection: &mut AsyncPgConnection,
        scope: &str,
    ) -> Result<ScopesModel, ScopeServiceError> {
        let scopes_list = scope
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let validated_scopes = scopes::table
            .select(scopes::name)
            .filter(scopes::name.eq_any(&scopes_list))
            .load(connection)
            .await
            .map_err(ScopeServiceError::from)?;

        Ok(ScopesModel {
            scopes: validated_scopes,
        })
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
