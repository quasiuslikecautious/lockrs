use yew::prelude::*;

use crate::models::UserDetailsModel;

#[derive(Clone, PartialEq)]
pub struct UserDetailsRedirectCallbacks {
    pub on_logout_click: Callback<MouseEvent>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserDetailsViewProps {
    pub model: UserDetailsModel,
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
        html! {
            <>
                <h2>{ctx.props().user_id.clone()}</h2>

                <button onclick={ctx.props().redirect_callbacks.on_logout_click.clone()}>
                    <p>{ "Logout" }</p>
                </button>
            </>
        }
    }
}
