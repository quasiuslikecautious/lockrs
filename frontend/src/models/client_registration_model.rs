use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClientRegistrationModel {
    pub application_name: String,
    pub application_description: String,
    pub application_type: String,
    pub homepage_url: String,
    pub redirect_url: String,

    #[serde(skip_serializing)]
    pub application_name_error: Option<String>,
    #[serde(skip_serializing)]
    pub application_description_error: Option<String>,
    #[serde(skip_serializing)]
    pub application_type_error: Option<String>,
    #[serde(skip_serializing)]
    pub homepage_url_error: Option<String>,
    #[serde(skip_serializing)]
    pub redirect_url_error: Option<String>,
}

impl ClientRegistrationModel {
    pub fn new() -> Self {
        Self {
            application_name: String::new(),
            application_description: String::new(),
            application_type: String::new(),
            homepage_url: String::new(),
            redirect_url: String::new(),

            application_name_error: None,
            application_description_error: None,
            application_type_error: None,
            homepage_url_error: None,
            redirect_url_error: None,
        }
    }
}
