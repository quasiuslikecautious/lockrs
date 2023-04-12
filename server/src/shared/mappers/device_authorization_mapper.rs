use crate::{
    db::models::DbDeviceAuthorization, 
    oauth2::models::DeviceAuthorizationModel,
};

use super::ScopeMapper;

pub struct DeviceAuthorizationMapper;

impl DeviceAuthorizationMapper {
    pub fn from_db(db_model: DbDeviceAuthorization) -> DeviceAuthorizationModel {
        DeviceAuthorizationModel {
            client_id: db_model.client_id,
            user_code: db_model.user_code,
            device_code: db_model.device_code,
            scopes: ScopeMapper::db_list_to_vec(&db_model.scopes),
            expires_at: db_model.expires_at,
        }
    }
}

