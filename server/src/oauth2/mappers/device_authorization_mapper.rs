use crate::{oauth2::models::DeviceAuthorizationModel, pg::models::PgDeviceAuthorization};

use super::ScopeMapper;

pub struct DeviceAuthorizationMapper;

impl DeviceAuthorizationMapper {
    pub fn from_db(db_model: PgDeviceAuthorization) -> DeviceAuthorizationModel {
        DeviceAuthorizationModel {
            client_id: db_model.client_id,
            user_code: db_model.user_code,
            device_code: db_model.device_code,
            scopes: ScopeMapper::db_list_to_vec(&db_model.scopes),
            expires_at: db_model.expires_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};

    #[test]
    fn it_should_map_db() {
        let client_id = String::from("CLIENT_ID");
        let user_code = String::from("1234-BCDF");
        let device_code = String::from("DEVICE_CODE");
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::minutes(1);
        let scopes = vec![Some(String::from("read")), Some(String::from("write"))];

        let db_auth = DbDeviceAuthorization {
            id: 1,
            client_id: client_id.clone(),
            user_code: user_code.clone(),
            device_code: device_code.clone(),
            created_at,
            expires_at,
            scopes,
        };

        let actual_auth = DeviceAuthorizationMapper::from_db(db_auth);

        let expected_auth = DeviceAuthorizationModel {
            client_id,
            user_code,
            device_code,
            expires_at,
            scopes: vec![String::from("read"), String::from("write")],
        };

        assert_eq!(actual_auth, expected_auth);
    }
}
