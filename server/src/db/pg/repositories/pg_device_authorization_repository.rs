use std::sync::Arc;

use async_trait::async_trait;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    oauth2::{
        mappers::DeviceAuthorizationMapper,
        models::{DeviceAuthorizationCreateModel, DeviceAuthorizationModel},
    },
    pg::{models::PgDeviceAuthorization, schema::device_authorizations},
    repositories::{DeviceAuthorizationRepository, DeviceAuthorizationRepositoryError},
    DbContext,
};

pub struct PgDeviceAuthorizationRepository {
    db_context: Arc<DbContext>,
}

impl PgDeviceAuthorizationRepository {
    pub fn new(db_context: &Arc<DbContext>) -> Self {
        Self {
            db_context: Arc::clone(db_context),
        }
    }
}

#[async_trait]
impl DeviceAuthorizationRepository for PgDeviceAuthorizationRepository {
    async fn create(
        &self,
        device_authorization_create: &DeviceAuthorizationCreateModel,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| DeviceAuthorizationRepositoryError::BadConnection)?;

        let pg_device_authorization = diesel::insert_into(device_authorizations::table)
            .values((
                device_authorizations::client_id.eq(&device_authorization_create.client_id),
                device_authorizations::user_code.eq(&device_authorization_create.user_code),
                device_authorizations::device_code.eq(&device_authorization_create.device_code),
                device_authorizations::expires_at.eq(&device_authorization_create.expires_at),
                device_authorizations::scopes.eq(&device_authorization_create.scopes),
            ))
            .get_result::<PgDeviceAuthorization>(conn)
            .await
            .map_err(|_| DeviceAuthorizationRepositoryError::NotCreated)?;

        Ok(DeviceAuthorizationMapper::from_pg(pg_device_authorization))
    }

    async fn get_by_user_code(
        &self,
        code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError> {
        let now = Utc::now().naive_utc();

        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| DeviceAuthorizationRepositoryError::BadConnection)?;

        let pg_device_authorization = device_authorizations::table
            .filter(device_authorizations::device_code.eq(code))
            .filter(device_authorizations::created_at.lt(now))
            .filter(device_authorizations::expires_at.gt(now))
            .first::<PgDeviceAuthorization>(conn)
            .await
            .map_err(|_| DeviceAuthorizationRepositoryError::NotFound)?;

        Ok(DeviceAuthorizationMapper::from_pg(pg_device_authorization))
    }

    async fn get_by_device_code(
        &self,
        code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError> {
        let now = Utc::now().naive_utc();

        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| DeviceAuthorizationRepositoryError::BadConnection)?;

        let pg_device_authorization = device_authorizations::table
            .filter(device_authorizations::user_code.eq(code))
            .filter(device_authorizations::created_at.lt(now))
            .filter(device_authorizations::expires_at.gt(now))
            .first::<PgDeviceAuthorization>(conn)
            .await
            .map_err(|_| DeviceAuthorizationRepositoryError::NotFound)?;

        Ok(DeviceAuthorizationMapper::from_pg(pg_device_authorization))
    }

    async fn delete_by_device_code(
        &self,
        _id: &str,
    ) -> Result<(), DeviceAuthorizationRepositoryError> {
        todo!();
    }
}
