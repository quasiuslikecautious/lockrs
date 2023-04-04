use base64::{Engine as _, engine::general_purpose};
use regex::Regex;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    models::LoginModel,
    views::{
        LoginView,
        LoginRedirectCallbacks,
        LoginFormCallbacks
    },
    Route,
};

pub enum LoginMessage {
    EmailUpdated(Event),
    PasswordUpdated(Event),

    SignupButtonClicked,
    SubmitButtonClicked,
}

pub struct LoginController {
    model: LoginModel,
    form_callbacks: LoginFormCallbacks,
    redirect_callbacks: LoginRedirectCallbacks
}

impl Component for LoginController {    
    type Message = LoginMessage;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            model: LoginModel::new(),
            form_callbacks: LoginFormCallbacks {
                on_submit: ctx.link().callback(|_| Self::Message::SubmitButtonClicked),
                on_email_change: ctx.link().callback(Self::Message::EmailUpdated),
                on_password_change: ctx.link().callback(Self::Message::PasswordUpdated),
            },
            redirect_callbacks: LoginRedirectCallbacks {
                on_signup_click: ctx.link().callback(|_| Self::Message::SignupButtonClicked),
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <LoginView
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
            Self::Message::SignupButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::SignupRoute);
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

impl LoginController {
    fn submit_form(&self) {
        spawn_local({
            let credentials = format!("{}:{}", &self.model.email, &self.model.password);
            let encoded = general_purpose::URL_SAFE_NO_PAD.encode(credentials);
            let basic_auth = format!("Basic {}", encoded);

            async move {
                Request::post("/api/v1/user/login")
                    .header("Authorization", &basic_auth)
                    .send()
                    .await
                    .expect("login request failed");
            }
        })
    }
}
