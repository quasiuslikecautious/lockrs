use serde::Deserialize;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

