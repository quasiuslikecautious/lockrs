use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::{
    establish_connection,
    models::DbUser,
    schema::users,
};

use crate::mappers::UserMapper;
use crate::models::{NewUser, User};

pub struct UserService;

impl UserService {
    pub fn create_user(new_user: NewUser) -> Result<User, UserServiceError> {
        let password_hash = hash(new_user.password, DEFAULT_COST)
            .map_err(|_| UserServiceError::HashError)?;
    
        let connection = &mut establish_connection();
        let result = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(users::table)
                    .values((
                        users::email.eq(&new_user.email),
                        users::password_hash.eq(password_hash)
                    ))
                    .get_result::<DbUser>(conn)
            });
    
        match result {
            Ok(db_user) => Ok(UserMapper::from_db(db_user)),
            Err(err) => Err(UserServiceError::from(err)),
        }
    }
    
    pub fn get_user_by_id(id: &Uuid) -> Result<User, UserServiceError> {
        let connection = &mut establish_connection();
        let result = users::table
            .filter(users::id.eq(id))
            .first::<DbUser>(connection);
        
        match result {
            Ok(db_user) => Ok(UserMapper::from_db(db_user)),
            Err(err) => Err(UserServiceError::from(err)),
        }
    }
    
    pub fn get_user_by_email(email: &str) -> Result<User, UserServiceError> {
        let connection = &mut establish_connection();
        let result = users::table
            .filter(users::email.eq(email))
            .first::<DbUser>(connection);
        
        match result {
            Ok(db_user) => Ok(UserMapper::from_db(db_user)),
            Err(err) => Err(UserServiceError::from(err)),
        }
    }
}

pub enum UserServiceError {
    AlreadyExistsError,
    DbError,
    HashError,
    NotFoundError,
}

impl From<diesel::result::Error> for UserServiceError {
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

