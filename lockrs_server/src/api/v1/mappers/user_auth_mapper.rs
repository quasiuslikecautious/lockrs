use crate::{api::v1::models::UserAuthModel, db::pg::models::PgUser, models::UserModel};

pub struct UserAuthMapper;

impl UserAuthMapper {
    pub fn from_pg(pg_user: PgUser) -> UserAuthModel {
        UserAuthModel {
            id: pg_user.id,
            email: pg_user.email,
            password_hash: pg_user.password_hash,
        }
    }

    pub fn into_user(user_auth: UserAuthModel) -> UserModel {
        UserModel {
            id: user_auth.id,
            email: user_auth.email,
        }
    }
}
