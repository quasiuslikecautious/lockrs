use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    models::UserDetailsModel,
    views::{UserDetailsView, UserDetailsRedirectCallbacks}, Route,
};

pub enum UserDetailsMessage {
    LogoutButtonClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserDetailsControllerProps {
    pub user_id: String,
}

pub struct UserDetailsController {
    model: UserDetailsModel,
    redirect_callbacks: UserDetailsRedirectCallbacks,
}

impl Component for UserDetailsController {
    type Message = UserDetailsMessage;
    type Properties = UserDetailsControllerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            model: UserDetailsModel::new(),
            redirect_callbacks: UserDetailsRedirectCallbacks {
                on_logout_click: ctx.link().callback(|_| Self::Message::LogoutButtonClicked),
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <UserDetailsView
                    model={self.model.clone()}
                    user_id={ctx.props().user_id.clone()}
                    redirect_callbacks={self.redirect_callbacks.clone()}
                />
            </>
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::LogoutButtonClicked => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::LogoutRoute);
            },
        }

        false
    }
}
