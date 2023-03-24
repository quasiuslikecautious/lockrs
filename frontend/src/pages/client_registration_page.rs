use yew::{function_component, html, Html};

use crate::styles;

#[function_component(ClientRegistrationPage)]
pub fn client_registration_page() -> Html {
    html! {
        <div class={ styles::form_styles() } id="client-registration-page">
            <div class="container" id="form-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Register a client" }</h2>
                <h4>{ "Fill out the required fields to register a client" }</h4>

                <form id="client-registration-form">
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="app-name" 
                            name="app-name" 
                            placeholder=" "
                        />
                        <div class="input-hint">
                            { "Application name" }
                        </div>
                    </div>
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="app-homepage" 
                            name="app-homepage" 
                            placeholder=" "
                        />
                        <div class="input-hint">
                            { "Homepage URL" }
                        </div>
                    </div>
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="app-description" 
                            name="app-description" 
                            placeholder=" "
                        />
                        <div class="input-hint">
                            { "Application Description" }
                        </div>
                    </div>
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="callback-url" 
                            name="callback-url" 
                            placeholder=" "
                        />
                        <div class="input-hint">
                            { "Authorization callback URL" }
                        </div>
                    </div>
                </form>
                <button>
                   <p>{ "Register" }</p>
                </button>
            </div>
        </div>
    }
}
