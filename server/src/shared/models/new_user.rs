use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

