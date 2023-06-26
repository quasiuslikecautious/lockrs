use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    oauth2::models::{ScopeCreateModel, ScopeModel},
    pg::schema::scopes,
    repositories::{ScopeRepository, ScopeRepositoryError},
    DbContext,
};

pub struct PgScopeRepository {
    db_context: Arc<DbContext>,
}

impl PgScopeRepository {
    pub fn new(db_context: &Arc<DbContext>) -> Self {
        Self {
            db_context: Arc::clone(db_context),
        }
    }
}

#[async_trait]
impl ScopeRepository for PgScopeRepository {
    async fn create(
        &self,
        _scope_create: &ScopeCreateModel,
    ) -> Result<ScopeModel, ScopeRepositoryError> {
        todo!();
    }

    async fn get_from_list(
        &self,
        scopes_list: &Vec<String>,
    ) -> Result<ScopeModel, ScopeRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| ScopeRepositoryError::BadConnection)?;

        let pg_scopes = scopes::table
            .select(scopes::name)
            .filter(scopes::name.eq_any(scopes_list))
            .load(conn)
            .await
            .map_err(|_| ScopeRepositoryError::NoneFound)?;

        Ok(ScopeModel { scopes: pg_scopes })
    }

    async fn delete_by_name(&self, _id: &str) -> Result<(), ScopeRepositoryError> {
        todo!();
    }
}
