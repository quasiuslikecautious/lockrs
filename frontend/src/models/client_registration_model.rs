use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormData {
    pub application_name: String,
    pub application_description: String,
    pub application_type: String,
    pub homepage_url: String,
    pub redirect_url: String,
}

impl FormData {
    pub fn new() -> Self {
        Self {
            application_name: String::new(),
            application_description: String::new(),
            application_type: String::new(),
            homepage_url: String::new(),
            redirect_url: String::new(),
        }
    }
}

#[derive(PartialEq)]
pub struct ClientRegistrationModel {
    pub char_count: usize,
    pub max_chars: usize,

    pub current_page_index: usize,
    pub num_pages: usize,
    pub page_hidden_states: Vec<bool>,

    pub next_button_hidden: bool,
    pub previous_button_hidden: bool,
    pub submit_button_hidden: bool,

    pub form_data: FormData,

    pub application_name_error: Option<String>,
    pub application_description_error: Option<String>,
    pub application_type_error: Option<String>,
    pub homepage_url_error: Option<String>,
    pub redirect_url_error: Option<String>,
}

impl ClientRegistrationModel {
    pub fn new() -> Self {
        let num_pages = 2;
        let mut page_hidden_states: Vec<bool> = Vec::with_capacity(num_pages);

        page_hidden_states.push(false);

        for _ in 1..num_pages {
            page_hidden_states.push(true);
        }

        Self {
            char_count: 0,
            max_chars: 300,

            current_page_index: 0,
            num_pages,
            page_hidden_states,

            next_button_hidden: false,
            previous_button_hidden: true,
            submit_button_hidden: true,
            
            form_data: FormData::new(),

            application_name_error: None,
            application_description_error: None,
            application_type_error: None,
            homepage_url_error: None,
            redirect_url_error: None,
        }
    }
}
