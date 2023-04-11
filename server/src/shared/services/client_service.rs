// use diesel::prelude::*;
// use uuid::Uuid;
// 
// use crate::db::{
//     establish_connection,
//     models::DbClient,
//     schema::clients,
// };

use crate::{
    models::Client,
    auth::requests::NewClientRequest,
};

pub struct ClientService;

impl ClientService {
    pub fn create_client(new_user: NewClientRequest) -> Result<Client, ClientServiceError> {
        todo!();
    }
    
    pub fn get_client_by_id(id: &str) -> Result<Client, ClientServiceError> {
        todo!();
    }
    
    pub fn get_clients_by_user(email: &str) -> Result<Vec<Client>, ClientServiceError> {
        todo!();
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
            diesel::result::Error::DatabaseError(error_kind, _) => {
                match error_kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => Self::AlreadyExistsError,
                    _ => Self::DbError,
                }
            }
            _ => Self::DbError,
        }
    }
}

