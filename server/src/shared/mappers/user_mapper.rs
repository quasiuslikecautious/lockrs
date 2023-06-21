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

    pub fn into_db(user: UserModel) -> DbUser {
        DbUser {
            id: user.id,
            email: user.email,
            password_hash: user.password_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn it_should_map_db() {
        let id = Uuid::new_v4();
        let email = String::from("user@localhost.com");
        let password_hash = String::from("PASSWORD_HASH");

        let db_user = DbUser {
            id,
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        let actual_user = UserMapper::from_db(db_user);

        let expected_user = UserModel {
            id,
            email,
            password_hash,
        };

        assert_eq!(actual_user, expected_user);
    }
}
