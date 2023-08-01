use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{
        pg::{models::PgUser, schema::users},
        repositories::{QueryFailure, RepositoryError, UserRepository},
        DbContext,
    },
    mappers::UserMapper,
    models::{UserCreateModel, UserModel, UserUpdateModel},
};

pub struct PgUserRepository;

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        user_create: &UserCreateModel,
    ) -> Result<UserModel, RepositoryError> {
        tracing::trace!(method = "create", email = user_create.email);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_user = diesel::insert_into(users::table)
            .values((
                users::email.eq(&user_create.email),
                users::password_hash.eq(&user_create.password_hash),
            ))
            .get_result::<PgUser>(conn)
            .await
            .map_err(RepositoryError::map_diesel_create)?;

        Ok(UserMapper::from_pg(pg_user))
    }

    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<UserModel, RepositoryError> {
        tracing::trace!(method = "get_by_id", ?id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_user = users::table
            .filter(users::id.eq(id))
            .first::<PgUser>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(UserMapper::from_pg(pg_user))
    }

    async fn get_by_email(
        &self,
        db_context: &Arc<DbContext>,
        email: &str,
    ) -> Result<UserModel, RepositoryError> {
        tracing::trace!(method = "get_by_email", email);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_user = users::table
            .filter(users::email.eq(email))
            .first::<PgUser>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(UserMapper::from_pg(pg_user))
    }

    async fn update_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
        update_user: &UserUpdateModel,
    ) -> Result<UserModel, RepositoryError> {
        tracing::trace!(method = "update_by_id", ?id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(update_user)
            .get_result::<PgUser>(conn)
            .await
            .map_err(RepositoryError::map_diesel_update)?;

        Ok(UserMapper::from_pg(pg_user))
    }

    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<(), RepositoryError> {
        tracing::trace!(method = "delete_by_id", ?id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let rows_affected = diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(conn)
            .await
            .map_err(RepositoryError::map_diesel_delete)?;

        if rows_affected != 1 {
            let msg = format!(
                "Expected 1 row to be affected by delete, but found {}",
                rows_affected
            );

            tracing::error!(error = msg);
            return Err(RepositoryError::QueryFailed(QueryFailure::NotDeleted));
        }

        Ok(())
    }
}
