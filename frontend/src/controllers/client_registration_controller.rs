use std::{cell::RefCell, rc::Rc};

use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{KeyboardEvent, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::{
    models::ClientRegistrationModel,
    views::ClientRegistrationView,
};

pub enum ClientRegistrationMessage {
    ApplicationNameUpdated(Event),
    ApplicationDescriptionUpdated(KeyboardEvent),
    ApplicationTypeUpdated(Event),
    HomepageUrlUpdated(Event),
    RedirectUrlUpdated(Event),

    NextButtonClicked,
    PreviousButtonClicked,
    SubmitButtonClicked,
}

pub struct ClientRegistrationController {
    model: Rc<RefCell<ClientRegistrationModel>>,
}

impl Component for ClientRegistrationController {
    type Message = ClientRegistrationMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: Rc::new(RefCell::new(ClientRegistrationModel::new())),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let application_name_cb = ctx.link().callback(Self::Message::ApplicationNameUpdated);
        let application_description_cb = ctx.link().callback(Self::Message::ApplicationDescriptionUpdated);
        let application_type_cb = ctx.link().callback(Self::Message::ApplicationTypeUpdated);
        let homepage_url_cb = ctx.link().callback(Self::Message::HomepageUrlUpdated);
        let redirect_url_cb = ctx.link().callback(Self::Message::RedirectUrlUpdated);

        let next_button_cb = ctx.link().callback(|_| Self::Message::NextButtonClicked);
        let previous_button_cb = ctx.link().callback(|_| Self::Message::PreviousButtonClicked);
        let submit_button_cb = ctx.link().callback(|_| Self::Message::SubmitButtonClicked);

        html! {
            <ClientRegistrationView
                model={self.model.clone()}
                application_name_cb={application_name_cb}
                application_description_cb={application_description_cb}
                application_type_cb={application_type_cb}
                homepage_url_cb={homepage_url_cb}
                redirect_url_cb={redirect_url_cb}
                next_button_cb={next_button_cb}
                previous_button_cb={previous_button_cb}
                submit_button_cb={submit_button_cb}
            />
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut model = self.model.borrow_mut();

        match msg {
            Self::Message::ApplicationNameUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    model.form_data.application_name = input.value();
                }
            },
            Self::Message::ApplicationDescriptionUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

                let Some(input) = input
                else {
                    return false;
                };

                let value = input.value();

                model.char_count = value.len();
                model.form_data.application_description = value;
            },
            Self::Message::ApplicationTypeUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    model.form_data.application_type = input.value();
                }
            },
            Self::Message::HomepageUrlUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    model.form_data.homepage_url = input.value();
                }
            },
            Self::Message::RedirectUrlUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    model.form_data.redirect_url = input.value();
                }
            },

            Self::Message::NextButtonClicked => {
                let current_index = model.current_page_index;
                let next_index = current_index + 1;

                if next_index >= model.num_pages {
                    return false;
                }

                model.page_hidden_states[current_index] = true;
                model.page_hidden_states[next_index] = false;

                if next_index >= model.num_pages - 1 {
                    model.next_button_hidden = true;
                    model.submit_button_hidden = false;
                }

                if next_index > 0 {
                    model.previous_button_hidden = false;
                }

                model.current_page_index = next_index;
            },
            Self::Message::PreviousButtonClicked => {
                let current_index = model.current_page_index;
                let prev_index = (current_index + 1) % model.num_pages;

                model.page_hidden_states[current_index] = true;
                model.page_hidden_states[prev_index] = false;

                if prev_index <= model.num_pages - 2 {
                    model.next_button_hidden = false;
                    model.submit_button_hidden = true;
                }

                if prev_index == 0 {
                    model.previous_button_hidden = true;
                }

                model.current_page_index = prev_index;
            },
            Self::Message::SubmitButtonClicked => {
                if !(model.application_name_error == None &&
                     model.application_description_error == None &&
                     model.application_type_error == None &&
                     model.homepage_url_error == None &&
                     model.redirect_url_error == None)
                {
                    return false;
                }
                
                self.submit_form();
            },
        };

        true
    }
}

impl ClientRegistrationController {
    fn submit_form(&self) {
        spawn_local({
            let form_data = self.model.borrow().form_data.clone();

            async move {
                let body = serde_json::to_string(&form_data).unwrap();

                Request::put("/api/v1/client/create")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await
                    .expect("login request failed");
            }
        })
    }
}
