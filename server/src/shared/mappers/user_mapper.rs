use crate::{models::User, db::models::DbUser};

pub struct UserMapper;

impl UserMapper {
    pub fn from_db(db_user: DbUser) -> User {
        User {
            id: db_user.id,
            email: db_user.email,
            password_hash: db_user.password_hash,
        }
    }
}

