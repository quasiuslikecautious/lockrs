use chrono::NaiveDateTime;

pub struct DeviceAuthorizationModel {
    pub client_id: String,
    pub user_code: String,
    pub device_code: String,
    pub scopes: Vec<String>,
    pub expires_at: NaiveDateTime,
}
