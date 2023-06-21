use base64::{engine::general_purpose, Engine as _};
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncPgConnection, RunQueryDsl};
use ring::rand::{SecureRandom, SystemRandom};
use uuid::Uuid;

use crate::{
    mappers::ClientMapper,
    models::{ClientCreateModel, ClientModel, ClientUpdateModel},
    pg::{
        models::{PgClient, PgRedirectUri},
        schema::{clients, redirect_uris},
    },
};

pub struct ClientService;

impl ClientService {
    #[allow(clippy::all)]
    pub async fn create_client(
        connection: &mut AsyncPgConnection,
        new_client: ClientCreateModel,
    ) -> Result<ClientModel, ClientServiceError> {
        let id = Self::generate_random_string();
        let secret = match new_client.is_public {
            true => None,
            false => Some(Self::generate_random_string()),
        };

        let db_client = connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                async move {
                    let client = diesel::insert_into(clients::table)
                        .values((
                            clients::id.eq(&id),
                            clients::secret.eq(&secret),
                            clients::user_id.eq(new_client.user_id),
                            clients::is_public.eq(new_client.is_public),
                            clients::name.eq(new_client.name),
                            clients::description.eq(new_client.description),
                            clients::homepage_url.eq(new_client.homepage_url.to_string()),
                        ))
                        .get_result::<PgClient>(conn)
                        .await?;

                    diesel::insert_into(redirect_uris::table)
                        .values((
                            redirect_uris::client_id.eq(&id),
                            redirect_uris::uri.eq(new_client.redirect_url.to_string()),
                        ))
                        .get_result::<PgRedirectUri>(conn)
                        .await?;

                    Ok(client)
                }
                .scope_boxed()
            })
            .await
            .map_err(|err: diesel::result::Error| ClientServiceError::from(err))?;

        Ok(ClientMapper::from_db(db_client))
    }

    pub async fn get_client_by_id(
        connection: &mut AsyncPgConnection,
        id: &str,
    ) -> Result<ClientModel, ClientServiceError> {
        let db_client = clients::table
            .filter(clients::id.eq(id))
            .first::<PgClient>(connection)
            .await
            .map_err(ClientServiceError::from)?;

        Ok(ClientMapper::from_db(db_client))
    }

    pub async fn get_clients_by_user(
        connection: &mut AsyncPgConnection,
        user_id: &Uuid,
    ) -> Result<Vec<ClientModel>, ClientServiceError> {
        let clients = clients::table
            .filter(clients::user_id.eq(user_id))
            .load::<PgClient>(connection)
            .await
            .map_err(ClientServiceError::from)?;

        Ok(clients
            .into_iter()
            .map(ClientMapper::from_db)
            .collect::<Vec<ClientModel>>())
    }

    pub async fn update_client_by_id(
        connection: &mut AsyncPgConnection,
        client_id: &str,
        update_client: ClientUpdateModel,
    ) -> Result<ClientModel, ClientServiceError> {
        let db_client = diesel::update(clients::table)
            .filter(clients::id.eq(client_id))
            .set(update_client)
            .get_result::<PgClient>(connection)
            .await
            .map_err(ClientServiceError::from)?;

        Ok(ClientMapper::from_db(db_client))
    }

    pub async fn delete_client_by_id(
        connection: &mut AsyncPgConnection,
        client_id: &str,
    ) -> Result<ClientModel, ClientServiceError> {
        let db_client = diesel::delete(clients::table)
            .filter(clients::id.eq(client_id))
            .get_result::<PgClient>(connection)
            .await
            .map_err(ClientServiceError::from)?;

        Ok(ClientMapper::from_db(db_client))
    }

    pub fn generate_random_string() -> String {
        let mut buffer = [0u8; 24];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer)
    }
}

pub enum ClientServiceError {
    AlreadyExistsError,
    DbError,
    NotFoundError,
}

impl From<diesel::result::Error> for ClientServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::NotFoundError,
            diesel::result::Error::DatabaseError(error_kind, _) => match error_kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => Self::AlreadyExistsError,
                _ => Self::DbError,
            },
            _ => Self::DbError,
        }
    }
}
