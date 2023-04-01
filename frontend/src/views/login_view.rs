use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::{
    styles, 
    models::LoginModel,
};

#[derive(Clone, Properties, PartialEq)]
pub struct LoginViewProps {
    pub model: Rc<RefCell<LoginModel>>,
    pub email_onchange: Callback<Event>,
    pub password_onchange: Callback<Event>,
    pub signup_button_onclick: Callback<MouseEvent>,
    pub submit_button_onclick: Callback<MouseEvent>,
}

pub struct LoginView;

impl Component for LoginView {
    type Message = ();
    type Properties = LoginViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
        let model = ctx.props().model.borrow();

        html! {
            <> 
                <h2>{ "Login to your account" }</h2>
                <h4>{ "Enter your email and password" }</h4>

                <form id="login-form">
                    <div class="input-container">
                        <input
                            type="text"
                            id="email" 
                            name="email" 
                            placeholder=" "
                            onchange={ctx.props().email_onchange.clone()}
                            value={model.form_data.email.clone()}
                        />
                        <label for="email" class="input-hint">
                            { "Enter email" }
                        </label>
                    </div>
                    <div class="input-container">
                        <input 
                            type="password"
                            id="password"
                            name="password"
                            placeholder=" "
                            onchange={ctx.props().password_onchange.clone()}
                            value={model.form_data.password.clone()}
                        />
                        <label for="password" class="input-hint">
                            { "Enter password" }
                        </label>
                    </div>
                </form>
                <br/>
                <div class={ styles::button_pair() }>
                    <button 
                        class="secondary" 
                        onclick={ctx.props().signup_button_onclick.clone()}
                    >
                        <p>{ "Create account" }</p>
                    </button> 

                    <button onclick={ctx.props().submit_button_onclick.clone()}>
                        <p>{ "Continue" }</p>
                    </button>
                </div>
            </>
        }
    }
}