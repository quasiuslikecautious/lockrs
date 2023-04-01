use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormData {
    pub user_code: String,
}

impl FormData {
    pub fn new() -> Self {
        Self {
            user_code: String::new(),
        }
    }
}

#[derive(PartialEq)]
pub struct DeviceCodeModel {
    pub form_data: FormData,
    pub user_code_error: Option<String>,
}

impl DeviceCodeModel {
    pub fn new() -> Self {
        Self {
            form_data: FormData::new(),
            user_code_error: None,
        }
    }
}
