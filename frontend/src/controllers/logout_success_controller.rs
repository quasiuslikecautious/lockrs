use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    Route,
    models::LogoutSuccessModel,
    views::LogoutSuccessView,
};

pub enum LogoutSuccessMessage {
    LoginButtonClicked,
}

pub struct LogoutSuccessController {
    model: Rc<RefCell<LogoutSuccessModel>>,
}

impl Component for LogoutSuccessController {
    type Message = LogoutSuccessMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: Rc::new(RefCell::new(LogoutSuccessModel::new()))
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let login_button_cb = ctx.link().callback(|_| Self::Message::LoginButtonClicked);

        html! {
            <LogoutSuccessView
                model={self.model.clone()}
                login_button_cb={login_button_cb}
            />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::LoginButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Login);
            }
        }
        
        true
    }
}
