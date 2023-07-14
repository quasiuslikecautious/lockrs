use std::sync::Arc;

use async_trait::async_trait;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::{
        pg::{models::PgDeviceAuthorization, schema::device_authorizations},
        repositories::{DeviceAuthorizationRepository, RepositoryError},
        DbContext,
    },
    oauth2::{
        mappers::DeviceAuthorizationMapper,
        models::{DeviceAuthorizationCreateModel, DeviceAuthorizationModel},
    },
};

pub struct PgDeviceAuthorizationRepository;

#[async_trait]
impl DeviceAuthorizationRepository for PgDeviceAuthorizationRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        device_authorization_create: &DeviceAuthorizationCreateModel,
    ) -> Result<DeviceAuthorizationModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

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
            .map_err(|_| RepositoryError::NotCreated)?;

        Ok(DeviceAuthorizationMapper::from_pg(pg_device_authorization))
    }

    async fn get_by_user_code(
        &self,
        db_context: &Arc<DbContext>,
        code: &str,
    ) -> Result<DeviceAuthorizationModel, RepositoryError> {
        let now = Utc::now().naive_utc();

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let pg_device_authorization = device_authorizations::table
            .filter(device_authorizations::device_code.eq(code))
            .filter(device_authorizations::created_at.lt(now))
            .filter(device_authorizations::expires_at.gt(now))
            .first::<PgDeviceAuthorization>(conn)
            .await
            .map_err(|_| RepositoryError::NotFound)?;

        Ok(DeviceAuthorizationMapper::from_pg(pg_device_authorization))
    }

    async fn get_by_device_code(
        &self,
        db_context: &Arc<DbContext>,
        code: &str,
    ) -> Result<DeviceAuthorizationModel, RepositoryError> {
        let now = Utc::now().naive_utc();

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let pg_device_authorization = device_authorizations::table
            .filter(device_authorizations::user_code.eq(code))
            .filter(device_authorizations::created_at.lt(now))
            .filter(device_authorizations::expires_at.gt(now))
            .first::<PgDeviceAuthorization>(conn)
            .await
            .map_err(|_| RepositoryError::NotFound)?;

        Ok(DeviceAuthorizationMapper::from_pg(pg_device_authorization))
    }

    async fn delete_by_device_code(
        &self,
        _db_context: &Arc<DbContext>,
        _id: &str,
    ) -> Result<(), RepositoryError> {
        todo!();
    }
}
