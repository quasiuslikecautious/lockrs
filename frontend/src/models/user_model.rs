use regex::Regex;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserModel {
    pub email: String,
    pub password: String,

    #[serde(skip_serializing)]
    pub email_error: Option<String>,
    #[serde(skip_serializing)]
    pub password_error: Option<String>,
}

impl UserModel {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),

            email_error: None,
            password_error: None,
        }
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;

        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if email_regex.is_match(&self.email) {
            self.email_error = None;
        } else {
            self.email_error = Some(String::from("Invalid email"));
        }
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;

        let password_regex = Regex::new(r"^(.{8,})").unwrap();
        if password_regex.is_match(&self.password) {
            self.password_error = None;
        } else {
            self.password_error = Some(String::from("Invalid password"));
        }
    }

    pub fn validate(&self) -> bool {
        self.email_error.is_none() && self.password_error.is_none()
    }
}
