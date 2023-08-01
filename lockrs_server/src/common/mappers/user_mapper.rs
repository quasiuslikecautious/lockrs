use crate::{db::pg::models::PgUser, models::UserModel};

pub struct UserMapper;

impl UserMapper {
    pub fn from_pg(pg_user: PgUser) -> UserModel {
        UserModel {
            id: pg_user.id,
            email: pg_user.email,
        }
    }

    pub fn into_pg(user: UserModel, password_hash: String) -> PgUser {
        PgUser {
            id: user.id,
            email: user.email,
            password_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn it_should_map_pg() {
        let id = Uuid::new_v4();
        let email = String::from("user@localhost.com");
        let password_hash = String::from("PASSWORD_HASH");

        let pg_user = PgUser {
            id,
            email: email.clone(),
            password_hash,
        };

        let actual_user = UserMapper::from_pg(pg_user);

        let expected_user = UserModel { id, email };

        assert_eq!(actual_user, expected_user);
    }
}
