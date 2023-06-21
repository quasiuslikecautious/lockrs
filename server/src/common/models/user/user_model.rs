use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}
