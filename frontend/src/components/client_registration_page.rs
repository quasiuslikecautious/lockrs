use reqwasm::http::{Response, Request};
use serde::{Serialize};
use serde_json;
use std::ops::Deref;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    HtmlTextAreaElement,
    HtmlSelectElement,
    HtmlInputElement,
    HtmlFormElement, 
    HtmlDivElement, 
    HtmlButtonElement, 
    console,
};
use url::Url;
use yew::prelude::*;
use yew_router::{navigator, prelude::use_navigator};

use crate::styles;

#[derive(Debug, Serialize)]
pub struct ClientRegistrationForm {
    pub application_name: String,
    pub application_description: String,
    pub application_type: String,
    pub homepage_url: String,
    pub redirect_url: String,
}

impl TryFrom<HtmlFormElement> for ClientRegistrationForm {
    type Error = &'static str;

    fn try_from(form_element: HtmlFormElement) -> Result<Self, Self::Error> {
        // confirm elements exist in form
        let application_name_element = form_element.get_with_name("application-name").ok_or("missing application-name on form")?;
        let application_description_element = form_element.get_with_name("application-description").ok_or("missing application-description on form")?;
        let application_type_element = form_element.get_with_name("application-type").ok_or("missing application-type on form")?;
        let homepage_url_element = form_element.get_with_name("homepage-url").ok_or("missing homepage-url on form")?;
        let redirect_url_element = form_element.get_with_name("redirect-url").ok_or("missing redirect-url on form")?;

        // confirm expected types
        let application_name_input = application_name_element.dyn_into::<HtmlInputElement>().map_err(|_| "application-name is not an input element")?;
        let application_description_textarea = application_description_element.dyn_into::<HtmlTextAreaElement>().map_err(|_| "application-description is not an input element")?;
        let application_type_select = application_type_element.dyn_into::<HtmlSelectElement>().map_err(|_| "application-type is not an input element")?;
        let homepage_url_input = homepage_url_element.dyn_into::<HtmlInputElement>().map_err(|_| "homepage-url is not an input element")?;
        let redirect_url_input = redirect_url_element.dyn_into::<HtmlInputElement>().map_err(|_| "redirect-url is not an input element")?;

        Ok(Self {
            application_name: application_name_input.value(),
            application_description: application_description_textarea.value(),
            application_type: application_type_select.value(),
            homepage_url: homepage_url_input.value(),
            redirect_url: redirect_url_input.value(),
        })
    }
}

async fn submit_form(form: ClientRegistrationForm) -> Response {
    let body = serde_json::to_string(&form).unwrap();

    Request::post("/api/v1/user/login")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("login request failed") 
}

#[function_component(ClientRegistrationPage)]
pub fn client_registration_page() -> Html {
    let navigator = use_navigator();

    let max_chars: usize = 300;
    let char_count = use_state(|| 0);

    let description_ref = use_node_ref();
    
    let description_onkeyup = {
        let char_count = char_count.clone();
        let description_ref = description_ref.clone();

        Callback::from(move |_| {
            let description = description_ref
                .cast::<HtmlTextAreaElement>()
                .expect("description_ref not attached to textarea element");
            
            if description.text_length() >= max_chars as u32 {
                let current_text = description.value();
                let trimmed_text = &current_text[0..max_chars];
                description.set_value(trimmed_text);
            }

            char_count.set(description.text_length());
        })
    };

    let form_page_count = 2;

    let next_button_ref = use_node_ref();
    let prev_button_ref = use_node_ref();
    let submit_button_ref = use_node_ref();

    let mut form_page_refs: Vec<NodeRef> = Vec::new();

    for _ in 0..form_page_count {
        form_page_refs.push(NodeRef::default());
    }

    let next_page_onclick = {
        let form_page_refs = form_page_refs.clone();
        let next_button_ref = next_button_ref.clone();
        let prev_button_ref = prev_button_ref.clone();
        let submit_button_ref = submit_button_ref.clone();

        Callback::from(move |_e: MouseEvent| {
            let mut to_be_focused = false;
            
            for i in 0..form_page_refs.len() {
                let page = form_page_refs[i]
                    .cast::<HtmlDivElement>()
                    .expect("page_ref not attached to div element");

                if to_be_focused {
                    page.set_hidden(false);

                    if i > 0 {
                        let prev_button = prev_button_ref
                            .cast::<HtmlButtonElement>()
                            .expect("prev_button_ref not attached to button element");

                        prev_button.set_hidden(false);
                    }
                    
                    if i == form_page_refs.len() - 1 {
                        let next_button = next_button_ref
                            .cast::<HtmlButtonElement>()
                            .expect("next_button_ref not attached to button element");

                        let submit_button = submit_button_ref
                            .cast::<HtmlButtonElement>()
                            .expect("submit_button_ref not attached to button element");

                        next_button.set_hidden(true);
                        submit_button.set_hidden(false);
                    }

                    break;
                }

                if !page.hidden() {
                    page.set_hidden(true);
                    to_be_focused = true;
                }
            }
        })
    };

    let prev_page_onclick = {
        let form_page_refs = form_page_refs.clone();
        let next_button_ref = next_button_ref.clone();
        let prev_button_ref = prev_button_ref.clone();
        let submit_button_ref = submit_button_ref.clone();

        Callback::from(move |_e: MouseEvent| {
            let mut to_be_focused = false;
            
            for i in (0..form_page_refs.len()).rev() {
                let page = form_page_refs[i]
                    .cast::<HtmlDivElement>()
                    .expect("page_ref not attached to div element");

                if to_be_focused {
                    page.set_hidden(false);

                    if i == 0 {
                        let prev_button = prev_button_ref
                            .cast::<HtmlButtonElement>()
                            .expect("prev_button_ref not attached to button element");

                        prev_button.set_hidden(true);
                    }
                    
                    if i == form_page_count - 2 {
                        let next_button = next_button_ref
                            .cast::<HtmlButtonElement>()
                            .expect("next_button_ref not attached to button element");

                        let submit_button = submit_button_ref
                            .cast::<HtmlButtonElement>()
                            .expect("submit_button_ref not attached to button element");

                        next_button.set_hidden(false);
                        submit_button.set_hidden(true);
                    }

                    break;
                }

                if !page.hidden() {
                    page.set_hidden(true);
                    to_be_focused = true;
                }
            }
        })
    };

    let form_ref = use_node_ref();

    let submit_onclick = {
        let form_ref = form_ref.clone();

        Callback::from(move |_e: MouseEvent| {
            let form_element = form_ref
                .cast::<HtmlFormElement>()
                .expect("form_ref not attached to form element");

            let form = match ClientRegistrationForm::try_from(form_element) {
                Ok(x) => x,
                Err(e) => {
                    console::log_1(&JsValue::from_str(e));
                    return;
                }
            };

            spawn_local({
                let navigator = navigator.clone();

                async move {
                    let res = submit_form(form).await;

                    if res.ok() {
                        // navigate to client page
                        console::log_1(&JsValue::from_str("client created"));
                    } else {
                        // invalidate form and publish message to page
                    }
                }
            });
        })
    };

    html! {
        <>
            <h2>{ "Register a client" }</h2>
            <h4>{ "Fill out the required fields to register a client" }</h4>

            <form ref={form_ref} id="client-registration-form">
                <div ref={form_page_refs[0].clone()} id="page-1">
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="application-name" 
                            name="application-name" 
                            placeholder=" "
                        />
                        <label for="application-name" class="input-hint">
                            { "Application name" }
                        </label>
                    </div>
                    <div class="input-container">
                        <textarea 
                            ref={description_ref}
                            id="application-description"
                            class="large-text-input"
                            name="application-description" 
                            placeholder=" "
                            onkeyup={description_onkeyup}
                        />
                        <label for="application-description" class="input-hint">
                            { "Application Description" }
                        </label>
                        <div id="char-counter">
                            <span id="counter">{ format!("{} / {}", char_count.deref(), max_chars) }</span>
                        </div>
                    </div>
                </div>
                <div ref={form_page_refs[1].clone()} id="page-2" hidden=true>
                    <div class="input-container">
                        <select
                            id="application-type"
                            name="application-type"
                            required=true
                        >
                            <option value="" disabled=true selected=true hidden=true></option>
                            <option value="public">{ "Native" }</option>
                            <option value="public">{ "Single-Page App" }</option>
                            <option value="public">{ "Web" }</option>
                            <option value="confidential">{ "Service" }</option> 
                        </select>
                        <label for="application-type" class="input-hint">
                            { "Application Type" }
                        </label>
                    </div>
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="homepage-url" 
                            name="homepage-url" 
                            placeholder=" "
                        />
                        <label for="homepage-url" class="input-hint">
                            { "Homepage URL" }
                        </label>
                    </div>
                    <div class="input-container">
                        <input 
                            type="text" 
                            id="redirect-url" 
                            name="redirect-url" 
                            placeholder=" "
                        />
                        <label for="redirect-url" class="input-hint">
                            { "Authorization callback URL" }
                        </label>
                    </div>
                </div>
            </form>
            <div class={styles::button_pair()}>
                <button 
                    ref={prev_button_ref.clone()}
                    class="secondary"
                    onclick={prev_page_onclick}
                    hidden=true
                >
                    <p>{ "Back" }</p>
                </button>

                <button 
                    ref={next_button_ref.clone()} 
                    onclick={next_page_onclick}
                >
                    <p>{ "Continue" }</p>
                </button>

                <button 
                    ref={submit_button_ref.clone()} 
                    onclick={submit_onclick}
                    hidden=true
                >
                    <p>{ "Submit" }</p>
                </button>
            </div>
        </>
    }
}
