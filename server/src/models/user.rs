use url::Url;
use uuid::Uuid;

use crate::{
    auth_response::{Result, Rejection},
    db::{DbError, models::DbUser},
};


#[derive(Debug)]
pub struct UserCredentials {
    id: Uuid,
}

impl UserCredentials {
    pub fn new(id: &Uuid) -> Self {
        Self {
            id: id.clone(),
        }
    }

    pub fn validate(&self, redirect_uri: &Url) -> Result<User> {
        DbUser::get_from_id(&self.id) 
            .map_err(|err| {
                match err {
                    DbError::NotFound       => Rejection::AccessDenied(redirect_uri.clone()),
                    DbError::InternalError  => Rejection::ServerError(None),
                }
            })?;

        Ok(User{
            id: self.id,
        })
    }
}

#[derive(Clone, Debug)]
pub struct User {
    id: Uuid,
}

impl User {
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}

