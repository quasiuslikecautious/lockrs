use yew::prelude::{function_component, html, Html};

use crate::styles;

#[function_component(LogoutPage)]
pub fn logout_page() -> Html {
    html! {
        <>
            <h2>{ "Hope to see you soon!" }</h2>
            <h4>{ "Are you sure you want to log out of your account?" }</h4>
            <br/>
            <div class={ styles::confirm_button_pair() }>
                <button>
                   <p>{ "Yes, I'm sure" }</p>
                </button>
                <button class="secondary">
                    <p>{ "No" }</p>
                </button> 
            </div>
        </>    
    }
}

