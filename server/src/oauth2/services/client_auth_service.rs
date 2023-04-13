use diesel::prelude::*;

use crate::{
    db::{establish_connection, models::DbClient, schema::clients},
    mappers::ClientMapper,
    models::ClientModel,
};

pub struct ClientAuthService;

impl ClientAuthService {
    pub fn verify_credentials(
        id: &str,
        secret: &Option<String>,
    ) -> Result<ClientModel, ClientAuthServiceError> {
        let mut query = clients::table.into_boxed().filter(clients::id.eq(&id));

        if let Some(secret) = secret {
            query = query.filter(clients::secret.eq(secret));
        }

        let connection = &mut establish_connection();
        let db_client = query
            .first::<DbClient>(connection)
            .map_err(ClientAuthServiceError::from)?;

        Ok(ClientMapper::from_db(db_client))
    }
}

pub enum ClientAuthServiceError {
    DbError,
    NotFoundError,
}

impl From<diesel::result::Error> for ClientAuthServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::NotFoundError,
            _ => Self::DbError,
        }
    }
}
