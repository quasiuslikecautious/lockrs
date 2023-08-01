use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    api::v1::{
        mappers::UserAuthMapper,
        models::{UserAuthModel, UserRegisterModel},
    },
    db::{
        pg::{models::PgUser, schema::users},
        repositories::{RepositoryError, UserAuthRepository},
        DbContext,
    },
};

pub struct PgUserAuthRepository;

#[async_trait]
impl UserAuthRepository for PgUserAuthRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        user_create: &UserRegisterModel,
    ) -> Result<UserAuthModel, RepositoryError> {
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

        Ok(UserAuthMapper::from_pg(pg_user))
    }

    async fn get_by_email(
        &self,
        db_context: &Arc<DbContext>,
        email: &str,
    ) -> Result<UserAuthModel, RepositoryError> {
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

        Ok(UserAuthMapper::from_pg(pg_user))
    }
}
