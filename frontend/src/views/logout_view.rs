use yew::prelude::*;

use crate::{models::UserModel, styles};

#[derive(Clone, PartialEq)]
pub struct LogoutRedirectCallbacks {
    pub on_logout_click: Callback<MouseEvent>,
    pub on_cancel_click: Callback<MouseEvent>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LoginViewProps {
    pub model: UserModel,
    pub redirect_callbacks: LogoutRedirectCallbacks,
}

pub struct LogoutView;

impl Component for LogoutView {
    type Message = ();
    type Properties = LoginViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ "Hope to see you soon!" }</h2>
                <h4>{ "Are you sure you want to log out of your account?" }</h4>
                <br/>
                <div class={ styles::button_pair() }>
                    <button onclick={ctx.props().redirect_callbacks.on_logout_click.clone()}>
                        <p>{ "Yes, I'm sure" }</p>
                    </button>
                    <button class="secondary" onclick={ctx.props().redirect_callbacks.on_cancel_click.clone()}>
                        <p>{ "No" }</p>
                    </button>
                </div>
            </>
        }
    }
}
