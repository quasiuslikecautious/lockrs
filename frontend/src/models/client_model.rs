use serde::Serialize;
use url::Url;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClientModel {
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

impl ClientModel {
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

    pub fn set_application_name(&mut self, app_name: String) {
        self.application_name = app_name;

        self.application_name_error = None;
    }

    pub fn set_application_description(&mut self, app_description: String) {
        self.application_description = app_description;

        if self.application_description.len() > 300 {
            self.application_description_error =
                Some(String::from("Description exceeds max length"));
        } else {
            self.application_description_error = None;
        }
    }

    pub fn set_application_type(&mut self, app_type: String) {
        self.application_type = app_type;

        self.application_type_error = match self.application_type.as_str() {
            "confidential" | "public" => None,
            _ => Some(String::from("Invalid application type selected")),
        };
    }

    pub fn set_homepage_url(&mut self, url: String) {
        self.homepage_url = url;

        if Url::parse(self.homepage_url.as_str()).is_ok() {
            self.homepage_url_error = None;
        } else {
            self.homepage_url_error = Some(String::from("Invalid homepage url"));
        }
    }

    pub fn set_redirect_url(&mut self, url: String) {
        self.redirect_url = url;

        if Url::parse(self.redirect_url.as_str()).is_ok() {
            self.redirect_url_error = None;
        } else {
            self.redirect_url_error = Some(String::from("Invalid OAuth redirect url"));
        }
    }

    pub fn validate(&self) -> bool {
        self.application_name_error.is_none()
            && self.application_description_error.is_none()
            && self.application_type_error.is_none()
            && self.homepage_url_error.is_none()
            && self.redirect_url_error.is_none()
    }
}
