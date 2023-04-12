use yew::prelude::*;

use crate::{models::ScopeModel, styles};

#[derive(Clone, Properties, PartialEq)]
pub struct ScopeConfirmationViewProps {
    pub model: ScopeModel,
}

pub struct ScopeConfirmationView;

impl Component for ScopeConfirmationView {
    type Message = ();
    type Properties = ScopeConfirmationViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ "Confirm requested scopes" }</h2>
                <h4>{ "Review the permissions being requested before approving this request" }</h4>

                <br/>
                <div class={ styles::button_pair() }>
                    <button>
                    <p>{ "Confirm" }</p>
                    </button>
                    <button class="secondary">
                        <p>{ "Cancel" }</p>
                    </button>
                </div>
            </>
        }
    }
}
