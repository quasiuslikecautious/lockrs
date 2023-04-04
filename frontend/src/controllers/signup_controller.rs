use base64::{Engine as _, engine::general_purpose};
use regex::Regex;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    models::SignupModel,
    views::{
        SignupView,
        SignupFormCallbacks,
        SignupRedirectCallbacks
    },
    Route,
};

pub enum SignupMessage {
    EmailUpdated(Event),
    PasswordUpdated(Event),

    LoginButtonClicked,
    SubmitButtonClicked,
}

pub struct SignupController {
    model: SignupModel,
    form_callbacks: SignupFormCallbacks,
    redirect_callbacks: SignupRedirectCallbacks,
}

impl Component for SignupController {
    type Message = SignupMessage;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            model: SignupModel::new(),
            form_callbacks: SignupFormCallbacks { 
                on_submit: ctx.link().callback(|_| Self::Message::SubmitButtonClicked),
                on_email_change: ctx.link().callback(Self::Message::EmailUpdated), 
                on_password_change: ctx.link().callback(Self::Message::PasswordUpdated), 
            },
            redirect_callbacks: SignupRedirectCallbacks {
                on_login_click: ctx.link().callback(|_| Self::Message::LoginButtonClicked),
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <SignupView
                model={self.model.clone()}
                form_callbacks={self.form_callbacks.clone()}
                redirect_callbacks={self.redirect_callbacks.clone()}
            />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::EmailUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                let Some(input) = input
                else {
                    return false;
                };

                self.model.email = input.value();

                let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

                if email_regex.is_match(&input.value()) {
                    self.model.email_error = None;
                } else {
                    self.model.email_error = Some(String::from("Invalid email"));
                }
            },
            Self::Message::PasswordUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                let Some(input) = input
                else {
                    return false;
                };

                self.model.password = input.value();

                let password_regex = Regex::new(r"^(.{8,})").unwrap();

                if password_regex.is_match(&input.value()) {
                    self.model.password_error = None;
                } else {
                    self.model.password_error = Some(String::from("Invalid password"));
                }
            },
            Self::Message::LoginButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::LoginRoute);
            },
            Self::Message::SubmitButtonClicked => {
                if !(self.model.email_error == None &&
                     self.model.password_error == None)
                {
                    return false;
                }

                self.submit_form();
            },
        };

        true
    }
}

impl SignupController {
    fn submit_form(&self) {
        spawn_local({
            let credentials = format!("{}:{}", &self.model.email, &self.model.password);
            let encoded = general_purpose::URL_SAFE_NO_PAD.encode(credentials);
            let basic_auth = format!("Basic {}", encoded);

            async move {

                Request::put("/api/v1/user/create")
                    .header("Authorization", &basic_auth)
                    .send()
                    .await
                    .expect("signup request failed");
            }
        })
    }
}
