use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    mappers::ClientMapper,
    models::ClientModel,
    pg::{models::PgClient, schema::clients},
};

pub struct ClientAuthService;

impl ClientAuthService {
    pub async fn verify_credentials(
        connection: &mut AsyncPgConnection,
        id: &str,
        secret: &Option<String>,
    ) -> Result<ClientModel, ClientAuthServiceError> {
        let mut query = clients::table.into_boxed().filter(clients::id.eq(&id));

        if let Some(secret) = secret {
            query = query.filter(clients::secret.eq(secret));
        }

        let db_client = query
            .first::<PgClient>(connection)
            .await
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
