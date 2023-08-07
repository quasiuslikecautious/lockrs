use crate::{api::v1::models::UserAuthModel, db::pg::models::PgUser, models::UserModel};

pub struct UserAuthMapper;

impl UserAuthMapper {
    pub fn from_pg(pg_user: PgUser) -> UserAuthModel {
        UserAuthModel::new(
            &pg_user.id,
            pg_user.email.as_str(),
            pg_user.password_hash.as_str(),
        )
    }

    pub fn into_user(user_auth: UserAuthModel) -> UserModel {
        UserModel::new(user_auth.id, user_auth.email.as_str())
    }
}
