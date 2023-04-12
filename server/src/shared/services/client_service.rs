use base64::{engine::general_purpose, Engine as _};
use diesel::prelude::*;
use ring::rand::{SecureRandom, SystemRandom};
use uuid::Uuid;

use crate::{
    db::{
        establish_connection,
        models::{DbClient, DbRedirectUri},
        schema::{clients, redirect_uris},
    },
    mappers::ClientMapper,
    models::{ClientCreateModel, ClientModel, ClientUpdateModel},
};

pub struct ClientService;

impl ClientService {
    #[allow(clippy::all)]
    pub fn create_client(new_client: ClientCreateModel) -> Result<ClientModel, ClientServiceError> {
        let id = Self::generate_random_string();
        let secret = match new_client.is_public {
            true => None,
            false => Some(Self::generate_random_string()),
        };

        let connection = &mut establish_connection();
        let db_client = connection
            .build_transaction()
            .read_write()
            .run(|conn| {
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
                    .get_result::<DbClient>(conn)?;

                diesel::insert_into(redirect_uris::table)
                    .values((
                        redirect_uris::client_id.eq(&id),
                        redirect_uris::uri.eq(new_client.redirect_url.to_string()),
                    ))
                    .get_result::<DbRedirectUri>(conn)?;

                Ok(client)
            })
            .map_err(|err: diesel::result::Error| ClientServiceError::from(err))?;

        Ok(ClientMapper::from_db(db_client))
    }

    pub fn get_client_by_id(id: &str) -> Result<ClientModel, ClientServiceError> {
        let connection = &mut establish_connection();
        let db_client = clients::table
            .filter(clients::id.eq(id))
            .first::<DbClient>(connection)
            .map_err(ClientServiceError::from)?;

        Ok(ClientMapper::from_db(db_client))
    }

    pub fn get_clients_by_user(user_id: &Uuid) -> Result<Vec<ClientModel>, ClientServiceError> {
        let connection = &mut establish_connection();
        let clients = clients::table
            .filter(clients::user_id.eq(user_id))
            .load::<DbClient>(connection)
            .map_err(ClientServiceError::from)?;

        Ok(clients
            .into_iter()
            .map(ClientMapper::from_db)
            .collect::<Vec<ClientModel>>())
    }

    pub fn update_client_by_id(
        client_id: &str,
        update_client: ClientUpdateModel,
    ) -> Result<ClientModel, ClientServiceError> {
        let connection = &mut establish_connection();
        let db_client = connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                diesel::update(clients::table)
                    .filter(clients::id.eq(client_id))
                    .set(update_client)
                    .get_result::<DbClient>(conn)
            })
            .map_err(ClientServiceError::from)?;

        Ok(ClientMapper::from_db(db_client))
    }

    pub fn delete_client_by_id(client_id: &str) -> Result<ClientModel, ClientServiceError> {
        let connection = &mut establish_connection();
        let db_client = connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                diesel::delete(clients::table)
                    .filter(clients::id.eq(client_id))
                    .get_result::<DbClient>(conn)
            })
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
