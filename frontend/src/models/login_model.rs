use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LoginModel {
    pub email: String,
    pub password: String,

    #[serde(skip_serializing)]
    pub email_error: Option<String>,
    #[serde(skip_serializing)]
    pub password_error: Option<String>,
}

impl LoginModel {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),

            email_error: None,
            password_error: None,
        }
    }
}
