use chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub struct DeviceAuthorizationModel {
    pub id: i32,
    pub client_id: String,
    pub user_code: String,
    pub device_code: String,
    pub scopes: Vec<String>,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct DeviceAuthorizationCreateModel {
    pub client_id: String,
    pub user_code: String,
    pub device_code: String,
    pub expires_at: NaiveDateTime,
    pub scopes: Vec<String>,
}
