use bcrypt::verify;

use crate::{
    auth::models::SessionModel,
    services::{UserService, UserServiceError}, models::UserAuthModel,
};

pub struct UserAuthService;

impl UserAuthService {
    pub fn login(user_auth: &UserAuthModel) -> Result<SessionModel, UserAuthServiceError> {
        let user = match UserService::get_user_by_email(&user_auth.email) {
            Ok(user) => user,
            Err(err) => match err {
                UserServiceError::NotFoundError => {
                    return Err(UserAuthServiceError::InvalidEmailError)
                }
                _ => return Err(UserAuthServiceError::DbError),
            },
        };

        let valid_password =
            verify(&user_auth.password, &user.password_hash).map_err(|_| UserAuthServiceError::HashError)?;

        if !valid_password {
            return Err(UserAuthServiceError::InvalidPasswordError);
        }

        Ok(SessionModel {
            id: user.id.to_string(),
            token: String::from("TODO"),
        })
    }

    pub fn logout(session_id: &str) -> Result<SessionModel, UserAuthServiceError> {
        todo!();
    }
}

pub enum UserAuthServiceError {
    DbError,
    HashError,
    InvalidEmailError,
    InvalidPasswordError,
    NotFoundError,
}

impl From<diesel::result::Error> for UserAuthServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::NotFoundError,
            _ => Self::DbError,
        }
    }
}
