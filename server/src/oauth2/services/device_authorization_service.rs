use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use ring::rand::{SecureRandom, SystemRandom};

use crate::{
    db::{repositories::DeviceAuthorizationRepository, DbContext},
    oauth2::models::{DeviceAuthorizationCreateModel, DeviceAuthorizationModel, ScopeModel},
};

pub struct DeviceAuthorizationService;

impl DeviceAuthorizationService {
    pub async fn create_device_authorization(
        db_context: &Arc<DbContext>,
        device_authorization_repository: &dyn DeviceAuthorizationRepository,
        client_id: &str,
        scopes_model: ScopeModel,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationServiceError> {
        let expires_at = (Utc::now() + Duration::minutes(5)).naive_utc();

        let device_authorization_create = DeviceAuthorizationCreateModel {
            client_id: client_id.to_string(),
            user_code: Self::generate_user_code(),
            device_code: Self::generate_device_code(),
            expires_at,
            scopes: scopes_model.scopes,
        };

        device_authorization_repository
            .create(db_context, &device_authorization_create)
            .await
            .map_err(|_| DeviceAuthorizationServiceError::NotCreated)
    }

    pub async fn get_from_device_code(
        db_context: &Arc<DbContext>,
        device_authorization_repository: &dyn DeviceAuthorizationRepository,
        device_code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationServiceError> {
        device_authorization_repository
            .get_by_device_code(db_context, device_code)
            .await
            .map_err(|_| DeviceAuthorizationServiceError::NotFound)
    }

    pub async fn get_from_user_code(
        db_context: &Arc<DbContext>,
        device_authorization_repository: &dyn DeviceAuthorizationRepository,
        user_code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationServiceError> {
        device_authorization_repository
            .get_by_user_code(db_context, user_code)
            .await
            .map_err(|_| DeviceAuthorizationServiceError::NotFound)
    }

    pub fn generate_user_code() -> String {
        const ALPHABET: &[u8] = b"0123456789BCDFGHJKLMMNPQRSTVWXZ";
        const CODE_LEN: usize = 8;

        let mut code = String::with_capacity(CODE_LEN);
        let mut buffer = [0u8; CODE_LEN];
        let rng = SystemRandom::new();

        rng.fill(&mut buffer).unwrap();

        for byte in buffer.iter() {
            let idx = byte % ALPHABET.len() as u8;
            let c = char::from(ALPHABET[idx as usize]);
            code.push(c);
        }

        code
    }

    pub fn generate_device_code() -> String {
        let mut buffer = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer)
    }
}

pub enum DeviceAuthorizationServiceError {
    NotCreated,
    NotFound,
}
