use crate::{db::pg::models::PgDeviceAuthorization, oauth2::models::DeviceAuthorizationModel};

use super::ScopeMapper;

pub struct DeviceAuthorizationMapper;

impl DeviceAuthorizationMapper {
    pub fn from_pg(pg_model: PgDeviceAuthorization) -> DeviceAuthorizationModel {
        DeviceAuthorizationModel {
            id: pg_model.id,
            client_id: pg_model.client_id,
            user_code: pg_model.user_code,
            device_code: pg_model.device_code,
            scopes: ScopeMapper::pg_list_to_vec(&pg_model.scopes),
            expires_at: pg_model.expires_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};

    #[test]
    fn it_should_map_pg() {
        let id = 1;
        let client_id = String::from("CLIENT_ID");
        let user_code = String::from("1234-BCDF");
        let device_code = String::from("DEVICE_CODE");
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::minutes(1);
        let scopes = vec![Some(String::from("read")), Some(String::from("write"))];

        let pg_auth = PgDeviceAuthorization {
            id,
            client_id: client_id.clone(),
            user_code: user_code.clone(),
            device_code: device_code.clone(),
            created_at,
            expires_at,
            scopes,
        };

        let actual_auth = DeviceAuthorizationMapper::from_pg(pg_auth);

        let expected_auth = DeviceAuthorizationModel {
            id,
            client_id,
            user_code,
            device_code,
            expires_at,
            scopes: vec![String::from("read"), String::from("write")],
        };

        assert_eq!(actual_auth, expected_auth);
    }
}
