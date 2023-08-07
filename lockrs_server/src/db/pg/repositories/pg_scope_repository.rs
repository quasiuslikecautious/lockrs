use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::{
        pg::schema::scopes,
        repositories::{RepositoryError, ScopeRepository},
        DbContext,
    },
    oauth2::v1::models::{ScopeCreateModel, ScopeModel},
};

pub struct PgScopeRepository;

#[async_trait]
impl ScopeRepository for PgScopeRepository {
    async fn create(
        &self,
        _db_context: &Arc<DbContext>,
        _scope_create: &ScopeCreateModel,
    ) -> Result<ScopeModel, RepositoryError> {
        tracing::trace!(method = "create");

        todo!();
    }

    async fn get_from_list(
        &self,
        db_context: &Arc<DbContext>,
        scopes_list: &[String],
    ) -> Result<ScopeModel, RepositoryError> {
        tracing::trace!(
            method = "get_from_list",
            scopes = ?scopes_list
        );

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_scopes = scopes::table
            .select(scopes::name)
            .filter(scopes::name.eq_any(scopes_list))
            .load(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(ScopeModel::new(pg_scopes.as_slice()))
    }

    async fn delete_by_name(
        &self,
        _db_context: &Arc<DbContext>,
        _id: &str,
    ) -> Result<(), RepositoryError> {
        tracing::trace!(method = "delete_by_name",);

        todo!();
    }
}
