use chrono::NaiveDateTime;

#[derive(PartialEq)]
pub struct DeviceAuthorizationModel {
    pub id: i32,
    pub client_id: String,
    pub user_code: String,
    pub device_code: String,
    pub scopes: Vec<String>,
    pub expires_at: NaiveDateTime,
}

impl DeviceAuthorizationModel {
    pub fn new(
        id: i32,
        client_id: &str,
        user_code: &str,
        device_code: &str,
        expires_at: &NaiveDateTime,
        scopes: &[String],
    ) -> Self {
        Self {
            id,
            client_id: client_id.to_owned(),
            user_code: user_code.to_owned(),
            device_code: device_code.to_owned(),
            expires_at: expires_at.to_owned(),
            scopes: scopes.to_vec(),
        }
    }
}

impl std::fmt::Debug for DeviceAuthorizationModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeviceAuthorizationCreateModel: {{ {:?}, {:?}, user_code: ********, device_code: ********, {:?}, {:?} }}",
            self.id,
            self.client_id,
            self.expires_at,
            self.scopes,
        )
    }
}

pub struct DeviceAuthorizationCreateModel {
    pub client_id: String,
    pub user_code: String,
    pub device_code: String,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}

impl DeviceAuthorizationCreateModel {
    pub fn new(
        client_id: &str,
        user_code: &str,
        device_code: &str,
        expires_at: &NaiveDateTime,
        scopes: &[String],
    ) -> Self {
        Self {
            client_id: client_id.to_owned(),
            user_code: user_code.to_owned(),
            device_code: device_code.to_owned(),
            expires_at: expires_at.to_owned(),
            scopes: scopes.to_vec(),
        }
    }
}

impl std::fmt::Debug for DeviceAuthorizationCreateModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeviceAuthorizationCreateModel: {{ {:?}, user_code: ********, device_code: ********, {:?}, {:?} }}",
            self.client_id,
            self.expires_at,
            self.scopes,
        )
    }
}
