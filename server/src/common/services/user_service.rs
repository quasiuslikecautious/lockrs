use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::pg::{models::PgUser, schema::users};

use crate::mappers::UserMapper;
use crate::models::{UserCreateModel, UserModel, UserUpdateModel};

pub struct UserService;

impl UserService {
    pub async fn create_user(
        connection: &mut AsyncPgConnection,
        new_user: UserCreateModel,
    ) -> Result<UserModel, UserServiceError> {
        let password_hash = Self::hash_password(new_user.password.as_str()).await?;

        let db_user = diesel::insert_into(users::table)
            .values((
                users::email.eq(&new_user.email),
                users::password_hash.eq(password_hash),
            ))
            .get_result::<PgUser>(connection)
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
            .first::<PgUser>(connection)
            .await?;

        Ok(UserMapper::from_db(db_user))
    }

    pub async fn get_user_by_email(
        connection: &mut AsyncPgConnection,
        email: &str,
    ) -> Result<UserModel, UserServiceError> {
        let db_user = users::table
            .filter(users::email.eq(email))
            .first::<PgUser>(connection)
            .await?;

        Ok(UserMapper::from_db(db_user))
    }

    pub async fn update_user_by_id(
        connection: &mut AsyncPgConnection,
        id: &Uuid,
        update_user: &UserUpdateModel,
    ) -> Result<UserModel, UserServiceError> {
        let mut orig_user = Self::get_user_by_id(connection, id).await?;

        if let Some(email) = &update_user.email {
            orig_user.email = email.to_string();
        }

        if let Some(password) = &update_user.password {
            orig_user.password_hash = Self::hash_password(password.as_str()).await?;
        }

        let db_user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(&UserMapper::into_db(orig_user))
            .get_result::<PgUser>(connection)
            .await?;

        Ok(UserMapper::from_db(db_user))
    }

    pub async fn delete_user_by_id(
        connection: &mut AsyncPgConnection,
        id: &Uuid,
    ) -> Result<bool, UserServiceError> {
        let rows_affected = diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(connection)
            .await?;

        Ok(rows_affected > 0)
    }

    pub async fn hash_password(password: &str) -> Result<String, UserServiceError> {
        let hash = hash(password, DEFAULT_COST).map_err(|_| UserServiceError::Hash)?;
        Ok(hash)
    }
}

pub enum UserServiceError {
    AlreadyExists,
    Db,
    Hash,
    NotFound,
}

impl From<diesel::result::Error> for UserServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::NotFound,
            diesel::result::Error::DatabaseError(error_kind, _) => match error_kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => Self::AlreadyExists,
                _ => Self::Db,
            },
            _ => Self::Db,
        }
    }
}
