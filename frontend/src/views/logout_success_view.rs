use yew::prelude::*;

use crate::models::LogoutSuccessModel;

#[derive(Clone, PartialEq)]
pub struct LogoutSuccessRedirectCallbacks {
    pub on_login_click: Callback<MouseEvent>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LogoutSuccessViewProps {
    pub model: LogoutSuccessModel,
    pub redirect_callbacks: LogoutSuccessRedirectCallbacks,
}

pub struct LogoutSuccessView;

impl Component for LogoutSuccessView {
    type Message = ();
    type Properties = LogoutSuccessViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ "Hope to see you soon!" }</h2>
                <h4>{ "You have successfully signed out of your account." }</h4>
                <br/>
                <button onclick={ctx.props().redirect_callbacks.on_login_click.clone()}>
                    <p>{ "Back to login" }</p>
                </button>
            </>
        }
    }
}
