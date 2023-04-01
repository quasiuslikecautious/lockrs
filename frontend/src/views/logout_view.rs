use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::{styles, models::LogoutModel};

#[derive(Clone, Properties, PartialEq)]
pub struct LoginViewProps {
    pub model: Rc<RefCell<LogoutModel>>,
    pub logout_button_onclick: Callback<MouseEvent>,
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
                    <button onclick={ctx.props().logout_button_onclick.clone()}>
                        <p>{ "Yes, I'm sure" }</p>
                    </button>
                    <button class="secondary">
                        <p>{ "No" }</p>
                    </button> 
                </div>
            </>    
        }
    }
}
