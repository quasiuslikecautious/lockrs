use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::db::{models::DbUser, schema::users};

use crate::mappers::UserMapper;
use crate::models::{UserCreateModel, UserModel};

pub struct UserService;

impl UserService {
    pub async fn create_user(
        connection: &mut AsyncPgConnection,
        new_user: UserCreateModel,
    ) -> Result<UserModel, UserServiceError> {
        let password_hash =
            hash(new_user.password, DEFAULT_COST).map_err(|_| UserServiceError::HashError)?;

        let db_user = diesel::insert_into(users::table)
            .values((
                users::email.eq(&new_user.email),
                users::password_hash.eq(password_hash),
            ))
            .get_result::<DbUser>(connection)
            .await
            .map_err(UserServiceError::from)?;

        Ok(UserMapper::from_db(db_user))
    }

    pub async fn get_user_by_id(
        connection: &mut AsyncPgConnection,
        id: &Uuid,
    ) -> Result<UserModel, UserServiceError> {
        let db_user = users::table
            .filter(users::id.eq(id))
            .first::<DbUser>(connection)
            .await
            .map_err(UserServiceError::from)?;

        Ok(UserMapper::from_db(db_user))
    }

    pub async fn get_user_by_email(
        connection: &mut AsyncPgConnection,
        email: &str,
    ) -> Result<UserModel, UserServiceError> {
        let db_user = users::table
            .filter(users::email.eq(email))
            .first::<DbUser>(connection)
            .await
            .map_err(UserServiceError::from)?;

        Ok(UserMapper::from_db(db_user))
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
            diesel::result::Error::DatabaseError(error_kind, _) => match error_kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => Self::AlreadyExistsError,
                _ => Self::DbError,
            },
            _ => Self::DbError,
        }
    }
}
