use base64::{engine::general_purpose, Engine as _};
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    models::UserModel,
    views::{LoginFormCallbacks, LoginRedirectCallbacks, LoginView},
    Route,
};

pub enum LoginMessage {
    EmailUpdated(Event),
    PasswordUpdated(Event),

    SignupButtonClicked,
    SubmitButtonClicked,
}

pub struct LoginController {
    model: UserModel,
    form_callbacks: LoginFormCallbacks,
    redirect_callbacks: LoginRedirectCallbacks,
}

impl Component for LoginController {
    type Message = LoginMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            model: UserModel::new(),
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

                self.model.set_email(input.value());
            }
            Self::Message::PasswordUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                let Some(input) = input
                else {
                    return false;
                };

                self.model.set_password(input.value());
            }
            Self::Message::SignupButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::SignupRoute);
            }
            Self::Message::SubmitButtonClicked => {
                if !self.model.validate() {
                    return false;
                }

                self.submit_form();
            }
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
