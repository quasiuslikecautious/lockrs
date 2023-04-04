use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    Route,
    models::UserModel,
    views::{LogoutSuccessView, LogoutSuccessRedirectCallbacks},
};

pub enum LogoutSuccessMessage {
    LoginButtonClicked,
}

pub struct LogoutSuccessController {
    model: UserModel,
    redirect_callbacks: LogoutSuccessRedirectCallbacks,
}

impl Component for LogoutSuccessController {
    type Message = LogoutSuccessMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            model: UserModel::new(),
            redirect_callbacks: LogoutSuccessRedirectCallbacks {
                on_login_click: ctx.link().callback(|_| Self::Message::LoginButtonClicked),
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <LogoutSuccessView
                model={self.model.clone()}
                redirect_callbacks={self.redirect_callbacks.clone()}
            />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::LoginButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::LoginRoute);
            }
        }
        
        true
    }
}
