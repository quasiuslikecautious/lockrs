use std::ops::Deref;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(ClientRegistrationPage)]
pub fn client_registration_page() -> Html {
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

    html! {
        <>
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
                    <label for="app-name" class="input-hint">
                        { "Application name" }
                    </label>
                </div>
                <div class="input-container">
                    <input 
                        type="text" 
                        id="app-homepage" 
                        name="app-homepage" 
                        placeholder=" "
                    />
                    <label for="app-homepage" class="input-hint">
                        { "Homepage URL" }
                    </label>
                </div>
                <div class="input-container">
                    <textarea 
                        ref={description_ref}
                        id="app-description"
                        class="large-text-input"
                        name="app-description" 
                        placeholder=" "
                        onkeyup={description_onkeyup}
                    />
                    <label for="app-description" class="input-hint">
                        { "Application Description" }
                    </label>
                    <div id="char-counter">
                        <span id="counter">{ format!("{} / {}", char_count.deref(), max_chars) }</span>
                    </div>
                </div>
                <div class="input-container">
                    <input 
                        type="text" 
                        id="callback-url" 
                        name="callback-url" 
                        placeholder=" "
                    />
                    <label for="callback-url" class="input-hint">
                        { "Authorization callback URL" }
                    </label>
                </div>
            </form>
            <button>
                <p>{ "Register" }</p>
            </button>
        </>
    }
}
