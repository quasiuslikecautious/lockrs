use std::ops::Deref;

use base64::{Engine as _, engine::general_purpose};
use regex::Regex;
use reqwasm::http::{Response, Request};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{styles, Route};

async fn submit_credentials(email: String, password: String) -> Response {
    let credentials = format!("{}:{}", &email, &password);
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(credentials);
    let basic_auth = format!("Basic {}", encoded);

    Request::post("/api/v1/user/login")
        .header("Authorization", &basic_auth)
        .send()
        .await
        .expect("login request failed") 
}

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let navigator = use_navigator().unwrap();

    let email_input_ref = use_node_ref();
    let password_input_ref = use_node_ref();

    let email_valid_class = use_state_eq(|| String::from(""));
    let password_valid_class = use_state_eq(|| String::from(""));

    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    let password_regex = Regex::new(r"^(.{8,})").unwrap();

    let email_onchange = {
        let email_regex = email_regex.clone();
        let email_valid_class = email_valid_class.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                if &input.value() != "" && !email_regex.is_match(&input.value()) {
                    email_valid_class.set(String::from("invalid"));
                    return;
                }
            }

            email_valid_class.set(String::from(""));
        })
    };
    
    let password_onchange = {
        let password_regex = password_regex.clone();
        let password_valid_class = password_valid_class.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                if &input.value() != "" && !password_regex.is_match(&input.value()) {
                    password_valid_class.set(String::from("invalid"));
                    return;
                }
            }

            password_valid_class.set(String::from(""));
        })
    };

    let login_onclick = {
        let navigator = navigator.clone();

        Callback::from(move |_| navigator.push(&Route::Login))
    };

    let signup_onclick = {
        let email_input_ref = email_input_ref.clone();
        let password_input_ref = password_input_ref.clone();
        let email_regex = email_regex.clone();
        let password_regex = password_regex.clone();

        Callback::from(move |_e: MouseEvent| {
            let Some(email_input) = email_input_ref.cast::<HtmlInputElement>()
            else {
                return;
            };

            let Some(password_input) = password_input_ref.cast::<HtmlInputElement>()
            else {
                return;
            };

            let valid_email = email_regex.is_match(&email_input.value());
            let valid_password = password_regex.is_match(&password_input.value());

            if !(valid_email && valid_password) {
                // TODO invalidate form and publish message to page
                return;
            }

            spawn_local({
                let navigator = navigator.clone();

                async move {
                    let res = submit_credentials(email_input.value(), password_input.value()).await;

                    if res.ok() {
                        navigator.push(&Route::ScopeConfirmation);
                    } else {
                        // invalidate form and publish message to page
                    }
                }
            });
        })
    };

    html! {
        <>
            <h2>{ "Create your account" }</h2>
            <h4>{ "Enter an email and password" }</h4>

             <form id="signup-form">
                <div class="input-container">
                    <input 
                        type="text" 
                        ref={email_input_ref}
                        id="email" 
                        class={ email_valid_class.deref() }
                        name="email" 
                        placeholder=" "
                        onchange={email_onchange}
                    />
                    <label for="email" class="input-hint">
                        { "Enter an email" }
                    </label>
                </div>
                <div class="input-container">
                    <input 
                        type="password"
                        ref={password_input_ref}
                        id="password"
                        class={ password_valid_class.deref() }
                        name="password"
                        placeholder=" "
                        onchange={password_onchange}
                    />
                    <label for="password" class="input-hint">
                        { "Enter a password" }
                    </label>
                </div>
            </form>
            <br/>
            <div class={ styles::button_pair() }>
                <button onclick={signup_onclick}>
                   <p>{ "Continue" }</p>
                </button>
                <button class="secondary" onclick={login_onclick}>
                    <p>{ "Login instead" }</p>
                </button> 
            </div>

        </>
    }
}
