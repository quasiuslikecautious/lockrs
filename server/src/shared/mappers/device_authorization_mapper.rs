use crate::{
    db::models::DbDeviceAuthorization, 
    oauth2::responses::DeviceAuthorizationResponse,
};

pub struct DeviceAuthorizationMapper;

impl DeviceAuthorizationMapper {
    pub fn from_db(db_device_authorization: DbDeviceAuthorization) -> DeviceAuthorizationResponse {
        DeviceAuthorizationResponse::new(
            db_device_authorization.user_code.as_str(),
            db_device_authorization.device_code.as_str()
        )
    }
}

