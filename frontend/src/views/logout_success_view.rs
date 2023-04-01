use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::models::LogoutSuccessModel;

#[derive(Clone, Properties, PartialEq)]
pub struct LogoutSuccessViewProps {
    pub model: Rc<RefCell<LogoutSuccessModel>>,
    pub login_button_cb: Callback<MouseEvent>,
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
                <button onclick={ctx.props().login_button_cb.clone()}>
                    <p>{ "Back to login" }</p>
                </button>
            </>
        }
    }
}
