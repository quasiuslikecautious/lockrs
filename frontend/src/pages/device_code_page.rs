use std::ops::Deref;

use regex::Regex;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::styles;

#[function_component(DevicePage)]
pub fn device_page() -> Html {
    let code_regex = Regex::new(r"^([b-df-hj-np-tv-xz0-9]{8})").unwrap();
    
    let code_input = use_state_eq(|| String::from(""));

    let code_onchange = {
        let code_input = code_input.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                if &input.value() != "" && !code_regex.is_match(&input.value()) {
                    code_input.set(String::from("invalid"));
                    return;
                }
            }

            code_input.set(String::from(""));
        })
    };

    html! {
        <div class={ styles::form_styles() } id="device-code-page">
            <div class="container" id="form-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Connect a device" }</h2>
                <h4>{ "Enter the code displayed on your device" }</h4>

                <form id="device-code-form">
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="device-code-input" 
                            class={code_input.deref()}
                            name="device-code" 
                            minlength="8" 
                            maxlength="8"
                            style="text-transform: uppercase;"
                            placeholder=" "
                            onchange={code_onchange}
                        />
                        <div class="input-hint">
                            { "Enter code" }
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
