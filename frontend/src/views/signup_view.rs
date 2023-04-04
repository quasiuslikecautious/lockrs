use yew::prelude::*;

use crate::{
    styles, 
    models::UserModel,
};

#[derive(Clone, PartialEq)]
pub struct SignupFormCallbacks {
    pub on_submit: Callback<MouseEvent>,
    pub on_email_change: Callback<Event>,
    pub on_password_change: Callback<Event>,
}

#[derive(Clone, PartialEq)]
pub struct SignupRedirectCallbacks {
    pub on_login_click: Callback<MouseEvent>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct SignupViewProps {
    pub model: UserModel,
    pub form_callbacks: SignupFormCallbacks,
    pub redirect_callbacks: SignupRedirectCallbacks,
}

pub struct SignupView;

impl Component for SignupView {
    type Message = ();
    type Properties = SignupViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
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
                            onchange={ctx.props().form_callbacks.on_email_change.clone()}
                            value={ctx.props().model.email.clone()}
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
                            onchange={ctx.props().form_callbacks.on_password_change.clone()}
                            value={ctx.props().model.password.clone()}
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
                        onclick={ctx.props().redirect_callbacks.on_login_click.clone()}
                    >
                        <p>{ "Login instead" }</p>
                    </button>

                    <button onclick={ctx.props().form_callbacks.on_submit.clone()}>
                       <p>{ "Continue" }</p>
                    </button>
                </div>
    
            </>
        }
    }
}
