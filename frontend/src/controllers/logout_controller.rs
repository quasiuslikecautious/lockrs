use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    Route,
    models::UserModel,
    views::{LogoutView, LogoutRedirectCallbacks},
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
            },
            Self::Message::CancelButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.back();
            },
        };

        true
    }
}

impl LogoutController {
    fn submit_logout(&self) {
        spawn_local(async move {
            Request::delete("/api/v1/session/:id")
                .send()
                .await
                .expect("logout request failed");
        })
    }
}
