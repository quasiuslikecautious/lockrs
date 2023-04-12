use crate::{db::models::DbUser, models::UserModel};

pub struct UserMapper;

impl UserMapper {
    pub fn from_db(db_user: DbUser) -> UserModel {
        UserModel {
            id: db_user.id,
            email: db_user.email,
            password_hash: db_user.password_hash,
        }
    }
}
