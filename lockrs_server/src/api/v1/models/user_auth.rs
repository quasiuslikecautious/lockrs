use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct UserAuthModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

pub struct UserLoginCredentials {
    pub email: String,
    pub password: String,
}

pub struct UserRegistration {
    pub email: String,
    pub password: String,
}

pub struct UserRegisterModel {
    pub email: String,
    pub password_hash: String,
}
