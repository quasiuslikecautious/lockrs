use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::{
    models::ClientRegistrationModel,
    styles,
};

#[derive(Clone, Properties, PartialEq)]
pub struct ClientRegistrationViewProps {
    pub model: Rc<RefCell<ClientRegistrationModel>>,

    pub application_name_onchange: Callback<Event>,
    pub application_description_onkeyup: Callback<KeyboardEvent>,
    pub application_type_onchange: Callback<Event>,
    pub homepage_url_onchange: Callback<Event>,
    pub redirect_url_onchange: Callback<Event>,

    pub next_button_onclick: Callback<MouseEvent>,
    pub previous_button_onclick: Callback<MouseEvent>,
    pub submit_button_onclick: Callback<MouseEvent>,
}

pub struct ClientRegistrationView;

impl Component for ClientRegistrationView {
    type Message = ();
    type Properties = ClientRegistrationViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>)-> Html {
        let model = ctx.props().model.borrow();

        html! {
            <>
                <h2>{ "Register a client" }</h2>
                <h4>{ "Fill out the required fields to register a client" }</h4>

                <form id="client-registration-form">
                    <div id="page-1" hidden={model.page_hidden_states[0]}>
                        <div class="input-container">
                            <input 
                                type="text" 
                                id="application-name" 
                                name="application-name" 
                                placeholder=" "
                                onchange={ctx.props().application_name_onchange.clone()}
                                value={model.form_data.application_name.clone()}
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
                                onkeyup={ctx.props().application_description_onkeyup.clone()}
                                value={model.form_data.application_description.clone()}
                            />
                            <label for="application-description" class="input-hint">
                                { "Application Description" }
                            </label>
                            <div id="char-counter">
                                <span id="counter">{ format!("{} / {}", model.char_count, model.max_chars) }</span>
                            </div>
                        </div>
                    </div>
                    <div id="page-2" hidden={model.page_hidden_states[1]}>
                        <div class="input-container">
                            <select
                                id="application-type"
                                name="application-type"
                                required=true
                                onchange={ctx.props().application_type_onchange.clone()}
                                value={model.form_data.application_type.clone()}
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
                                onchange={ctx.props().homepage_url_onchange.clone()}
                                value={model.form_data.homepage_url.clone()}
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
                                onchange={ctx.props().redirect_url_onchange.clone()}
                                value={model.form_data.redirect_url.clone()}
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
                        onclick={ctx.props().previous_button_onclick.clone()}
                        hidden={model.previous_button_hidden}
                    >
                        <p>{ "Back" }</p>
                    </button>

                    <button 
                        onclick={ctx.props().next_button_onclick.clone()}
                        hidden={model.next_button_hidden}
                    >
                        <p>{ "Continue" }</p>
                    </button>

                    <button 
                        onclick={ctx.props().submit_button_onclick.clone()}
                        hidden={model.submit_button_hidden}
                    >
                        <p>{ "Submit" }</p>
                    </button>
                </div>
            </>
        }
    }
}
