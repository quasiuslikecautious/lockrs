use yew::{html, function_component, Html};

use crate::styles;

#[function_component(ScopeConfirmationPage)]
pub fn scope_confirmation_page() -> Html {
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

