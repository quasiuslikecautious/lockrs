use yew::prelude::*;

use crate::{components::FormFieldContainer, models::UserModel, styles};

#[derive(Clone, PartialEq)]
pub struct LoginFormCallbacks {
    pub on_submit: Callback<SubmitEvent>,
    pub on_email_change: Callback<Event>,
    pub on_password_change: Callback<Event>,
}

#[derive(Clone, PartialEq)]
pub struct LoginRedirectCallbacks {
    pub on_signup_click: Callback<MouseEvent>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LoginViewProps {
    pub model: UserModel,
    pub form_callbacks: LoginFormCallbacks,
    pub redirect_callbacks: LoginRedirectCallbacks,
}

pub struct LoginView;

impl Component for LoginView {
    type Message = ();
    type Properties = LoginViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ "Login to your account" }</h2>
                <h4>{ "Enter your email and password" }</h4>

                <form
                    id="login-form"
                    onsubmit={ctx.props().form_callbacks.on_submit.clone()}
                >
                    <FormFieldContainer
                        name="email"
                        prompt="Enter email"
                        error={ctx.props().model.email_error.clone()}
                    >
                        <input
                            type="text"
                            id="email"
                            name="email"
                            placeholder=" "
                            onchange={ctx.props().form_callbacks.on_email_change.clone()}
                            value={ctx.props().model.email.clone()}
                        />
                    </FormFieldContainer>
                    <FormFieldContainer
                        name="password"
                        prompt="Enter password"
                        error={ctx.props().model.password_error.clone()}
                    >
                        <input
                            type="password"
                            id="password"
                            name="password"
                            placeholder=" "
                            onchange={ctx.props().form_callbacks.on_password_change.clone()}
                            value={ctx.props().model.password.clone()}
                        />
                    </FormFieldContainer>
                </form>
                <br/>
                <div class={ styles::button_pair() }>
                    <button
                        type="submit"
                        form="login-form"
                    >
                        <p>{ "Continue" }</p>
                    </button>

                    <button
                        class="secondary"
                        onclick={ctx.props().redirect_callbacks.on_signup_click.clone()}
                    >
                        <p>{ "Create account instead" }</p>
                    </button>

                </div>
            </>
        }
    }
}
