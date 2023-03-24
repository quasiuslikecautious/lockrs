use yew::{function_component, html, Html};

use crate::styles;

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    html! {
        <div class={ styles::form_styles() }>
            <div class="container" id="form-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Create your account" }</h2>
                <h4>{ "Enter an email and password" }</h4>

                 <form id="device-code-form">
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="email" 
                            name="email" 
                            placeholder=" "
                        />
                        <div class="input-hint">
                            { "Enter an email" }
                        </div>
                    </div>
                    <div class="input-container">
                        <input 
                            type="password"
                            id="password"
                            name="password"
                            placeholder=" "
                        />
                        <div class="input-hint">
                            { "Enter a password" }
                        </div>
                    </div>
                </form>
                <br/>
                <button>
                   <p>{ "Continue" }</p>
                </button>
            </div>
        </div>
    }
}
