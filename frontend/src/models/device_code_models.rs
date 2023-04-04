use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceCodeModel {
    pub user_code: String,

    #[serde(skip_serializing)]
    pub user_code_error: Option<String>,
}

impl DeviceCodeModel {
    pub fn new() -> Self {
        Self {
            user_code: String::new(),
            user_code_error: None,
        }
    }
}
