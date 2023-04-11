use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewUserRequest {
    pub email: String,
    pub password: String,
}

