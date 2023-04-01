use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::{
    styles, 
    models::SignupModel,
};

#[derive(Clone, Properties, PartialEq)]
pub struct SignupViewProps {
    pub model: Rc<RefCell<SignupModel>>,
    pub email_cb: Callback<Event>,
    pub password_cb: Callback<Event>,
    pub login_button_cb: Callback<MouseEvent>,
    pub submit_button_cb: Callback<MouseEvent>,
}

pub struct SignupView;

impl Component for SignupView {
    type Message = ();
    type Properties = SignupViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
        let model = ctx.props().model.borrow();

        html! {
            <>
                <h2>{ "Create your account" }</h2>
                <h4>{ "Enter an email and password" }</h4>
    
                 <form id="signup-form">
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="email" 
                            name="email" 
                            placeholder=" "
                            onchange={ctx.props().email_cb.clone()}
                            value={model.form_data.email.clone()}
                        />
                        <label for="email" class="input-hint">
                            { "Enter an email" }
                        </label>
                    </div>
                    <div class="input-container">
                        <input 
                            type="password"
                            id="password"
                            name="password"
                            placeholder=" "
                            onchange={ctx.props().password_cb.clone()}
                            value={model.form_data.password.clone()}
                        />
                        <label for="password" class="input-hint">
                            { "Enter a password" }
                        </label>
                    </div>
                </form>
                <br/>
                <div class={ styles::button_pair() }>
                    <button 
                        class="secondary" 
                        onclick={ctx.props().login_button_cb.clone()}
                    >
                        <p>{ "Login instead" }</p>
                    </button>

                    <button onclick={ctx.props().submit_button_cb.clone()}>
                       <p>{ "Continue" }</p>
                    </button>
                </div>
    
            </>
        }
    }
}
