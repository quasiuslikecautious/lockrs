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

    pub fn create_from_credentials(email: String, password: String) -> Result<User> {
        let db_user = DbUser::insert(&email, &password)
            .map_err(|_| Rejection::ServerError(None))?;

        Ok(User::from(db_user))
    }
    
    pub fn get_from_credentials(email: String, password: String) -> Result<User> {
        let db_user = DbUser::get_from_credentials(&email, &password)
            .map_err(|_| Rejection::ServerError(None))?;

        Ok(User::from(db_user))
    }

    pub fn validate(&self, redirect_uri: &Url) -> Result<User> {
        let db_user = DbUser::get_from_id(&self.id) 
            .map_err(|err| {
                match err {
                    DbError::NotFound       => Rejection::AccessDenied(redirect_uri.clone()),
                    DbError::InternalError  => Rejection::ServerError(None),
                }
            })?;

        Ok(User::from(db_user))
    }
}

#[derive(Clone, Debug)]
pub struct User {
    id: Uuid,
    email: String,
    password_hash: String,
}

impl User {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_password_hash(&self) -> &String {
        &self.password_hash
    }
}

impl From<DbUser> for User {
    fn from(db_user: DbUser) -> Self {
        Self {
            id: db_user.id,
            email: db_user.email,
            password_hash: db_user.password_hash,
        }
    }
}

