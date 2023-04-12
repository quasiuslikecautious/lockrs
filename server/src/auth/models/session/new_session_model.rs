use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewSessionModel {
    pub email: String,
    pub password: String,
}

