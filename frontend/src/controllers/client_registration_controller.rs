use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{KeyboardEvent, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::{
    models::ClientModel,
    views::{
        ClientRegistrationView, 
        ClientRegistrationFormCallbacks, 
    },
};

pub enum ClientRegistrationControllerMessage {
    ApplicationNameUpdated(Event),
    ApplicationDescriptionUpdated(KeyboardEvent),
    ApplicationTypeUpdated(Event),
    HomepageUrlUpdated(Event),
    RedirectUrlUpdated(Event),

    SubmitButtonClicked,
}

pub struct ClientRegistrationController {
    model: ClientModel,
    form_callbacks: ClientRegistrationFormCallbacks,
}

impl Component for ClientRegistrationController {
    type Message = ClientRegistrationControllerMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            model: ClientModel::new(),
            form_callbacks: ClientRegistrationFormCallbacks { 
                on_submit: ctx.link().callback(|_| Self::Message::SubmitButtonClicked), 
                on_application_name_change: ctx.link().callback(Self::Message::ApplicationNameUpdated), 
                on_application_description_keyup: ctx.link().callback(Self::Message::ApplicationDescriptionUpdated), 
                on_application_type_change: ctx.link().callback(Self::Message::ApplicationTypeUpdated), 
                on_homepage_url_change: ctx.link().callback(Self::Message::HomepageUrlUpdated), 
                on_redirect_url_change: ctx.link().callback(Self::Message::RedirectUrlUpdated), 
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <ClientRegistrationView
                description_max_len=300
                model={self.model.clone()}
                form_callbacks={self.form_callbacks.clone()}
            />
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ApplicationNameUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    self.model.application_name = input.value();
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

                self.model.application_description = value;
            },
            Self::Message::ApplicationTypeUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    self.model.application_type = input.value();
                }
            },
            Self::Message::HomepageUrlUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    self.model.homepage_url = input.value();
                }
            },
            Self::Message::RedirectUrlUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    self.model.redirect_url = input.value();
                }
            },
            Self::Message::SubmitButtonClicked => {
                if !(self.model.application_name_error == None &&
                     self.model.application_description_error == None &&
                     self.model.application_type_error == None &&
                     self.model.homepage_url_error == None &&
                     self.model.redirect_url_error == None)
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
            let body = serde_json::to_string(&self.model).unwrap();

            async move {

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
