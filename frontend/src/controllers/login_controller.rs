use std::{cell::RefCell, rc::Rc};

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
    views::LoginView, Route,
};

pub enum LoginMessage {
    EmailUpdated(Event),
    PasswordUpdated(Event),

    SignupButtonClicked,
    SubmitButtonClicked,
}

pub struct LoginController {
    model: Rc<RefCell<LoginModel>>
}

impl Component for LoginController {
    type Message = LoginMessage;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: Rc::new(RefCell::new(LoginModel::new())),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let email_onchange = ctx.link().callback(Self::Message::EmailUpdated);
        let password_onchange = ctx.link().callback(Self::Message::PasswordUpdated);
        
        let signup_button_onclick = ctx.link().callback(|_| Self::Message::SignupButtonClicked);
        let submit_button_onclick = ctx.link().callback(|_| Self::Message::SubmitButtonClicked);

        html! {
            <LoginView
                model={self.model.clone()}
                email_onchange={email_onchange}
                password_onchange={password_onchange}
                signup_button_onclick={signup_button_onclick}
                submit_button_onclick={submit_button_onclick}
            />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut model = self.model.borrow_mut();
        
        match msg {
            Self::Message::EmailUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                let Some(input) = input
                else {
                    return false;
                };

                model.form_data.email = input.value();

                let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

                if email_regex.is_match(&input.value()) {
                    model.email_error = None;
                } else {
                    model.email_error = Some(String::from("Invalid email"));
                }
            },
            Self::Message::PasswordUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                let Some(input) = input
                else {
                    return false;
                };

                model.form_data.password = input.value();

                let password_regex = Regex::new(r"^(.{8,})").unwrap();

                if password_regex.is_match(&input.value()) {
                    model.password_error = None;
                } else {
                    model.password_error = Some(String::from("Invalid password"));
                }
            },
            Self::Message::SignupButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::SignupRoute);
            },
            Self::Message::SubmitButtonClicked => {
                if !(model.email_error == None &&
                     model.password_error == None)
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
            let form_data = self.model.borrow().form_data.clone();

            async move {
                let credentials = format!("{}:{}", &form_data.email, &form_data.password);
                let encoded = general_purpose::URL_SAFE_NO_PAD.encode(credentials);
                let basic_auth = format!("Basic {}", encoded);

                Request::post("/api/v1/user/login")
                    .header("Authorization", &basic_auth)
                    .send()
                    .await
                    .expect("login request failed");
            }
        })
    }
}
