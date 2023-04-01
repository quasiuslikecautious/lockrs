use std::{cell::RefCell, rc::Rc};

use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    Route,
    models::LogoutModel,
    views::LogoutView,
};

pub enum LogoutMessage {
    LogoutButtonClicked,
}

pub struct LogoutController {
    model: Rc<RefCell<LogoutModel>>,
}

impl Component for LogoutController {
    type Message = LogoutMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: Rc::new(RefCell::new(LogoutModel::new()))
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let logout_button_cb = ctx.link().callback(|_| Self::Message::LogoutButtonClicked);

        html! {
            <LogoutView
                model={self.model.clone()}
                logout_button_cb={logout_button_cb}
            />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::LogoutButtonClicked => {
                self.submit_logout();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::LogoutSuccess);
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
