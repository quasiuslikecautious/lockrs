use serde::Serialize;
use url::Url;

#[derive(Debug, Serialize)]
pub struct DeviceAuthorizationResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: Url,
    pub interval: i32,
    pub expires_in: i32,
}

impl DeviceAuthorizationResponse {
    pub fn new(user_code: &str, device_code: &str) -> Self {
        Self {
            user_code: user_code.to_owned(),
            device_code: device_code.to_owned(),
            verification_uri: Url::parse("http://127.0.0.1:8080/device")
                .expect("Failed to parse device code verification url"),
            interval: 5000,
            expires_in: 30000,
        }
    }
}
