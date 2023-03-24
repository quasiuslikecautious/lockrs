use yew::prelude::{function_component, html, Html};

use crate::styles;

#[function_component(LogoutPage)]
pub fn logout_page() -> Html {
    html! {
        <div class={ styles::form_styles() }>
            <div class="container" id="form-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Hope to see you soon!" }</h2>
                <h4>{ "Are you sure you want to log out of your account?" }</h4>
                <br/>
                <div class={ styles::confirm_button_pair() }>
                    <button class="secondary">
                        <p>{ "No" }</p>
                    </button>
                    <button>
                       <p>{ "Yes, I'm sure" }</p>
                    </button>
                </div>
            </div>
        </div>
    }
}

