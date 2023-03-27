use std::ops::Deref;

use regex::Regex;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::styles;

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    let password_regex = Regex::new(r"^(.{8,})").unwrap();

    let email_input = use_state_eq(|| String::from(""));
    let password_input = use_state_eq(|| String::from(""));

    let email_onchange = {
        let email_input = email_input.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                if &input.value() != "" && !email_regex.is_match(&input.value()) {
                    email_input.set(String::from("invalid"));
                    return;
                }
            }

            email_input.set(String::from(""));
        })
    };
    
    let password_onchange = {
        let password_input = password_input.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                if &input.value() != "" && !password_regex.is_match(&input.value()) {
                    password_input.set(String::from("invalid"));
                    return;
                }
            }

            password_input.set(String::from(""));
        })
    };

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
                            class={ email_input.deref() }
                            name="email" 
                            placeholder=" "
                            onchange={email_onchange}
                        />
                        <div class="input-hint">
                            { "Enter an email" }
                        </div>
                    </div>
                    <div class="input-container">
                        <input 
                            type="password"
                            id="password"
                            class={ password_input.deref() }
                            name="password"
                            placeholder=" "
                            onchange={password_onchange}
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
