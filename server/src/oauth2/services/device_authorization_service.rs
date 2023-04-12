use base64::{Engine as _, engine::general_purpose};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use ring::rand::{SecureRandom, SystemRandom};

use crate::{
    db::{
        establish_connection,
        schema::device_authorizations, 
        models::DbDeviceAuthorization,
    }, 
    mappers::DeviceAuthorizationMapper,
    oauth2::models::{
        DeviceAuthorizationModel,
        ScopesModel,
    },
};

pub struct DeviceAuthorizationService;

impl DeviceAuthorizationService {
    pub fn create_device_authorization(
        client_id: &str,
        scopes: ScopesModel,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationServiceError> {
        let expires_at = (Utc::now() + Duration::minutes(5)).naive_utc();
        let scopes = scopes.scopes.into_iter().map(|s| Some(s)).collect::<Vec<Option<String>>>();

        let connection = &mut establish_connection();
        let result = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(device_authorizations::table)
                    .values((
                        device_authorizations::client_id.eq(client_id),
                        device_authorizations::user_code.eq(Self::generate_user_code()),
                        device_authorizations::device_code.eq(Self::generate_device_code()),
                        device_authorizations::expires_at.eq(expires_at),
                        device_authorizations::scopes.eq(scopes),
                    ))
                    .get_result::<DbDeviceAuthorization>(conn)
            })
            .map_err(|err| DeviceAuthorizationServiceError::from(err))?;

        Ok(DeviceAuthorizationMapper::from_db(result))
    }

    pub fn get_from_device_code(
        device_code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationServiceError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        let result = device_authorizations::table
            .filter(device_authorizations::device_code.eq(device_code))
            .filter(device_authorizations::created_at.lt(now))
            .filter(device_authorizations::expires_at.gt(now))
            .first::<DbDeviceAuthorization>(connection)
            .map_err(|err| DeviceAuthorizationServiceError::from(err))?;

        Ok(DeviceAuthorizationMapper::from_db(result))
    }

    pub fn get_from_user_code(
        user_code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationServiceError> {
        let now = Utc::now().naive_utc();

        let connection = &mut establish_connection();
        let result = device_authorizations::table
            .filter(device_authorizations::user_code.eq(user_code))
            .filter(device_authorizations::created_at.lt(now))
            .filter(device_authorizations::expires_at.gt(now))
            .first::<DbDeviceAuthorization>(connection)
            .map_err(|err| DeviceAuthorizationServiceError::from(err))?;

        Ok(DeviceAuthorizationMapper::from_db(result))
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
        general_purpose::URL_SAFE_NO_PAD.encode(buffer).to_string()

    } 
}

pub enum DeviceAuthorizationServiceError {
    DbError,
    NotFound,
}

impl From<diesel::result::Error> for DeviceAuthorizationServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::NotFound,
            _ => Self::DbError,
        }
    }
}

