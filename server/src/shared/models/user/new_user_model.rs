use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUserModel {
    pub email: String,
    pub password: String,
}

