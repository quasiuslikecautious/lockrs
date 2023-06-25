use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    mappers::UserMapper,
    models::{UserCreateModel, UserModel, UserUpdateModel},
    pg::{models::PgUser, schema::users},
    repositories::{UserRepository, UserRepositoryError},
    DbContext,
};

pub struct PgUserRepository {
    db_context: Arc<DbContext>,
}

impl PgUserRepository {
    pub fn new(db_context: &Arc<DbContext>) -> Self {
        Self {
            db_context: Arc::clone(db_context),
        }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(
        &self,
        user_create: &UserCreateModel,
    ) -> Result<UserModel, UserRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| UserRepositoryError::BadConnection)?;

        let pg_user = diesel::insert_into(users::table)
            .values((
                users::email.eq(&user_create.email),
                users::password_hash.eq(&user_create.password_hash),
            ))
            .get_result::<PgUser>(conn)
            .await
            .map_err(|_| UserRepositoryError::NotCreated)?;

        Ok(UserMapper::from_pg(pg_user))
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<UserModel, UserRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| UserRepositoryError::BadConnection)?;

        let pg_user = users::table
            .filter(users::id.eq(id))
            .first::<PgUser>(conn)
            .await
            .map_err(|_| UserRepositoryError::NotFound)?;

        Ok(UserMapper::from_pg(pg_user))
    }

    async fn get_by_email(&self, email: &str) -> Result<UserModel, UserRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| UserRepositoryError::BadConnection)?;

        let pg_user = users::table
            .filter(users::email.eq(email))
            .first::<PgUser>(conn)
            .await
            .map_err(|_| UserRepositoryError::NotFound)?;

        Ok(UserMapper::from_pg(pg_user))
    }

    async fn update_by_id(
        &self,
        id: &Uuid,
        update_user: &UserUpdateModel,
    ) -> Result<UserModel, UserRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| UserRepositoryError::BadConnection)?;

        let pg_user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(update_user)
            .get_result::<PgUser>(conn)
            .await
            .map_err(|_| UserRepositoryError::NotUpdated)?;

        Ok(UserMapper::from_pg(pg_user))
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), UserRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| UserRepositoryError::BadConnection)?;

        let rows_affected = diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(conn)
            .await
            .map_err(|_| UserRepositoryError::BadDelete)?;

        if rows_affected != 1 {
            return Err(UserRepositoryError::BadDelete);
        }

        Ok(())
    }
}
