use yew::prelude::*;

use crate::{components::FormFieldContainer, models::UserModel, styles};

#[derive(Clone, PartialEq)]
pub struct SignupFormCallbacks {
    pub on_submit: Callback<SubmitEvent>,
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ "Create your account" }</h2>
                <h4>{ "Enter an email and password" }</h4>

                 <form
                    id="signup-form"
                    onsubmit={ctx.props().form_callbacks.on_submit.clone()}
                >
                    <FormFieldContainer
                        name="email"
                        prompt="Enter an email"
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
                        prompt="Enter a password"
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
                        form="signup-form"
                    >
                       <p>{ "Continue" }</p>
                    </button>

                    <button
                        class="secondary"
                        onclick={ctx.props().redirect_callbacks.on_login_click.clone()}
                    >
                        <p>{ "Login instead" }</p>
                    </button>
                </div>

            </>
        }
    }
}
