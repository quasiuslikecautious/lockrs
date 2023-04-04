use yew::prelude::*;

use crate::{
    models::ClientModel,
    styles,
};

pub enum ClientRegistrationViewMessage {
    NextButtonClicked,
    PreviousButtonClicked,
}

#[derive(Clone, PartialEq)]
pub struct ClientRegistrationFormCallbacks {
    pub on_submit: Callback<MouseEvent>,
    pub on_application_name_change: Callback<Event>,
    pub on_application_description_keyup: Callback<KeyboardEvent>,
    pub on_application_type_change: Callback<Event>,
    pub on_homepage_url_change: Callback<Event>,
    pub on_redirect_url_change: Callback<Event>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct ClientRegistrationViewProps {
    pub description_max_len: usize,

    pub model: ClientModel,

    pub form_callbacks: ClientRegistrationFormCallbacks,
}

pub struct ClientRegistrationView {
    pub char_count: usize,
    pub max_chars: usize,

    pub current_page_index: usize,
    pub num_pages: usize,
    pub page_hidden_states: Vec<bool>,

    pub next_button_hidden: bool,
    pub previous_button_hidden: bool,
    pub submit_button_hidden: bool,
}

impl Component for ClientRegistrationView {
    type Message = ClientRegistrationViewMessage;
    type Properties = ClientRegistrationViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let num_pages = 2;
        let mut page_hidden_states: Vec<bool> = Vec::with_capacity(num_pages);

        page_hidden_states.push(false);

        for _ in 1..num_pages {
            page_hidden_states.push(true);
        }

        Self {
            char_count: 0,
            max_chars: ctx.props().description_max_len,

            current_page_index: 0,
            num_pages,
            page_hidden_states,

            next_button_hidden: false,
            previous_button_hidden: true,
            submit_button_hidden: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::NextButtonClicked => {
                let current_index = self.current_page_index;
                let next_index = current_index + 1;

                if next_index >= self.num_pages {
                    return false;
                }

                self.page_hidden_states[current_index] = true;
                self.page_hidden_states[next_index] = false;

                if next_index >= self.num_pages - 1 {
                    self.next_button_hidden = true;
                    self.submit_button_hidden = false;
                }

                if next_index > 0 {
                    self.previous_button_hidden = false;
                }

                self.current_page_index = next_index;
            },
            Self::Message::PreviousButtonClicked => {
                let current_index = self.current_page_index;
                let prev_index = (current_index + 1) % self.num_pages;

                self.page_hidden_states[current_index] = true;
                self.page_hidden_states[prev_index] = false;

                if prev_index <= self.num_pages - 2 {
                    self.next_button_hidden = false;
                    self.submit_button_hidden = true;
                }

                if prev_index == 0 {
                    self.previous_button_hidden = true;
                }

                self.current_page_index = prev_index;
            },
        };

        true
    }

    fn view(&self, ctx: &Context<Self>)-> Html {
        html! {
            <>
                <h2>{ "Register a client" }</h2>
                <h4>{ "Fill out the required fields to register a client" }</h4>

                <form id="client-registration-form">
                    <div id="page-1" hidden={self.page_hidden_states[0]}>
                        <div class="input-container">
                            <input 
                                type="text" 
                                id="application-name" 
                                name="application-name" 
                                placeholder=" "
                                onchange={ctx.props().form_callbacks.on_application_name_change.clone()}
                                value={ctx.props().model.application_name.clone()}
                            />
                            <label for="application-name" class="input-hint">
                                { "Application name" }
                            </label>
                        </div>
                        <div class="input-container">
                            <textarea 
                                id="application-description"
                                class="large-text-input"
                                name="application-description" 
                                placeholder=" "
                                onkeyup={ctx.props().form_callbacks.on_application_description_keyup.clone()}
                                value={ctx.props().model.application_description.clone()}
                            />
                            <label for="application-description" class="input-hint">
            { "Application Description" }
                            </label>
                            <div id="char-counter">
                                <span id="counter">{ format!("{} / {}", self.char_count, self.max_chars) }</span>
                            </div>
                        </div>
                    </div>
                    <div id="page-2" hidden={self.page_hidden_states[1]}>
                        <div class="input-container">
                            <select
                                id="application-type"
                                name="application-type"
                                required=true
                                onchange={ctx.props().form_callbacks.on_application_type_change.clone()}
                                value={ctx.props().model.application_type.clone()}
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
                                onchange={ctx.props().form_callbacks.on_homepage_url_change.clone()}
                                value={ctx.props().model.homepage_url.clone()}
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
                                onchange={ctx.props().form_callbacks.on_redirect_url_change.clone()}
                                value={ctx.props().model.redirect_url.clone()}
                            />
                            <label for="redirect-url" class="input-hint">
                                { "OAuth Redirect URL" }
                            </label>
                        </div>
                    </div>
                </form>
                <div class={styles::button_pair()}>
                    <button 
                        class="secondary"
                        onclick={ctx.link().callback(|_| Self::Message::PreviousButtonClicked)}
                        hidden={self.previous_button_hidden}
                    >
                        <p>{ "Back" }</p>
                    </button>

                    <button 
                        onclick={ctx.link().callback(|_| Self::Message::NextButtonClicked)}
                        hidden={self.next_button_hidden}
                    >
                        <p>{ "Continue" }</p>
                    </button>

                    <button 
                        onclick={ctx.props().form_callbacks.on_submit.clone()}
                        hidden={self.submit_button_hidden}
                    >
                        <p>{ "Submit" }</p>
                    </button>
                </div>
            </>
        }
    }
}
