use regex::Regex;
use yew::{function_component, html, use_state, Html, Callback};

use crate::styles;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    let email_input = use_state(|| 0);
    let email_onchange = {
        let email_input = email_input.clone();
        Callback::from(move |_| email_input.set(*email_input + 1))
    };

    html! {
        <div class={ styles::form_styles() id="login-page" }>
            <div class="container" id="form-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Login to your account" }</h2>
                <h4>{ "Enter your email and password" }</h4>

                <p>{ *email_input }</p>

                 <form id="login-form">
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="email" 
                            name="email" 
                            onchange={email_onchange}
                            placeholder=" "
                        />
                        <div class="input-hint">
                            { "Enter email" }
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
                            { "Enter password" }
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
