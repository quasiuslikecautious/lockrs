use yew::{function_component, html, Html};

use crate::styles;

#[function_component(LogoutSuccessPage)]
pub fn logout_success_page() -> Html {
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
                <h4>{ "You have successfully signed out of your account." }</h4>
                <br/>
                <button>
                    <p>{ "Back to login" }</p>
                </button>
            </div>
        </div>
    }
}
