use yew::prelude::*;

use crate::models::UserModel;

#[derive(Clone, PartialEq)]
pub struct UserDetailsRedirectCallbacks {
    pub on_logout_click: Callback<MouseEvent>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserDetailsViewProps {
    pub model: UserModel,
    pub user_id: String,
    pub redirect_callbacks: UserDetailsRedirectCallbacks,
}

pub struct UserDetailsView;

impl Component for UserDetailsView {
    type Message = ();
    type Properties = UserDetailsViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        html! {
            <>
                <h2>{props.user_id}</h2>

                <button onclick={props.redirect_callbacks.on_logout_click}>
                    <p>{ "Logout" }</p>
                </button>
            </>
        }
    }
}
