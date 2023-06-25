use chrono::NaiveDateTime;

pub struct DeviceAuthorizationCreateModel {
    pub client_id: String,
    pub user_code: String,
    pub device_code: String,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}
