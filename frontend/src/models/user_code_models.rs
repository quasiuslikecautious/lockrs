use regex::Regex;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserCodeModel {
    pub user_code: String,

    #[serde(skip_serializing)]
    pub user_code_error: Option<String>,
}

impl UserCodeModel {
    pub fn new() -> Self {
        Self {
            user_code: String::new(),

            user_code_error: None,
        }
    }

    pub fn set_user_code(&mut self, user_code: String) {
        self.user_code = user_code;

        let user_code_regex = Regex::new(r"^([b-df-hj-np-tv-xz0-9]{8})").unwrap();

        if user_code_regex.is_match(&self.user_code) {
            self.user_code_error = None;
        } else {
            self.user_code_error = Some(String::from("Invalid user code"));
        }
    }

    pub fn validate(&self) -> bool {
        self.user_code_error.is_none()
    }
}
