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
    oauth2::models::{ScopeCreateModel, ScopeModel},
};

pub struct PgScopeRepository;

#[async_trait]
impl ScopeRepository for PgScopeRepository {
    async fn create(
        &self,
        _db_context: &Arc<DbContext>,
        _scope_create: &ScopeCreateModel,
    ) -> Result<ScopeModel, RepositoryError> {
        todo!();
    }

    async fn get_from_list(
        &self,
        db_context: &Arc<DbContext>,
        scopes_list: &Vec<String>,
    ) -> Result<ScopeModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let pg_scopes = scopes::table
            .select(scopes::name)
            .filter(scopes::name.eq_any(scopes_list))
            .load(conn)
            .await
            .map_err(|_| RepositoryError::NotFound)?;

        Ok(ScopeModel { scopes: pg_scopes })
    }

    async fn delete_by_name(
        &self,
        _db_context: &Arc<DbContext>,
        _id: &str,
    ) -> Result<(), RepositoryError> {
        todo!();
    }
}
