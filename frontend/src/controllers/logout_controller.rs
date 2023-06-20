use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window, HtmlDocument};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    models::UserModel,
    views::{LogoutRedirectCallbacks, LogoutView},
    Route,
};

pub enum LogoutMessage {
    LogoutButtonClicked,
    CancelButtonClicked,
}

pub struct LogoutController {
    model: UserModel,
    redirect_callbacks: LogoutRedirectCallbacks,
}

impl Component for LogoutController {
    type Message = LogoutMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            model: UserModel::new(),
            redirect_callbacks: LogoutRedirectCallbacks {
                on_logout_click: ctx.link().callback(|_| Self::Message::LogoutButtonClicked),
                on_cancel_click: ctx.link().callback(|_| Self::Message::CancelButtonClicked),
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <LogoutView
                model={self.model.clone()}
                redirect_callbacks={self.redirect_callbacks.clone()}
            />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::LogoutButtonClicked => {
                self.submit_logout();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::LogoutSuccessRoute);
            }
            Self::Message::CancelButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.back();
            }
        };

        true
    }
}

impl LogoutController {
    fn get_cookie_value(name: &str) -> Option<String> {
        let document = window().and_then(|win| win.document())?;
        console::log_1(&"document grabbed".into());
        let html = document.dyn_into::<HtmlDocument>().ok()?;
        console::log_1(&"successful html parse".into());

        let cookies = html.cookie().ok()?;
        console::log_1(&"cookies grabbed".into());
        console::log_1(&cookies.clone().into());
        let cookie_pairs = cookies.split(';');
        console::log_1(&"cookies split".into());

        for pair in cookie_pairs {
            console::log_1(&pair.into());
            let parts = pair.split_once('=')?;
            let key = parts.0;

            console::log_1(&key.into());

            if key.trim() == name {
                let value = parts.1;
                console::log_1(&value.into());
                return Some(value.trim().to_string());
            }
        }

        None
    }

    fn submit_logout(&self) {
        let Some(session_id) = Self::get_cookie_value("s_id")
        else {
            console::log_1(&"bad cookie read".into());
            return;
        };

        spawn_local(async move {
            let route = format!("/api/v1/sessions/{}", session_id);
            Request::delete(route.as_str())
                .send()
                .await
                .expect("logout request failed");
        })
    }
}
