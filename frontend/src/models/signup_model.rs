use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormData {
    pub email: String,
    pub password: String,
}

impl FormData {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
        }
    }
}

#[derive(PartialEq)]
pub struct SignupModel {
    pub form_data: FormData,

    pub password_error: Option<String>,
    pub email_error: Option<String>,
}

impl SignupModel {
    pub fn new() -> Self {
        Self {
            form_data: FormData::new(),

            password_error: None,
            email_error: None,
        }
    }
}
