use std::ops::Deref;

use regex::Regex;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
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
        <> 
            <h2>{ "Login to your account" }</h2>
            <h4>{ "Enter your email and password" }</h4>

            <form id="login-form">
                <div class="input-container">
                    <input 
                        type="text" 
                        id="email" 
                        class={ email_input.deref() }
                        name="email" 
                        placeholder=" "
                        onchange={email_onchange}
                    />
                    <label for="email" class="input-hint">
                        { "Enter email" }
                    </label>
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
                    <label for="password" class="input-hint">
                        { "Enter password" }
                    </label>
                </div>
            </form>
            <br/>
            <button>
               <p>{ "Continue" }</p>
            </button>
        </>
    }
}
