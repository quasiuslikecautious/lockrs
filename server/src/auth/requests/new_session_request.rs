use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewSessionRequest {
    pub email: String,
    pub password: String,
}

