use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::models::DeviceCodeModel;

#[derive(Clone, Properties, PartialEq)]
pub struct DeviceCodeViewProps {
    pub model: Rc<RefCell<DeviceCodeModel>>,
    pub user_code_cb: Callback<Event>,
    pub submit_button_cb: Callback<MouseEvent>,
}

pub struct DeviceCodeView;

impl Component for DeviceCodeView {
    type Message = ();
    type Properties = DeviceCodeViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let model = ctx.props().model.borrow();

        html! {
            <>
                <h2>{ "Connect a device" }</h2>
                <h4>{ "Enter the code displayed on your device" }</h4>

                <form id="device-code-form">
                    <div class="input-container">
                        <input 
                            type="text"
                            id="user-code" 
                            name="user-code" 
                            minlength="8" 
                            maxlength="8"
                            style="text-transform: uppercase;"
                            placeholder=" "
                            onchange={ctx.props().user_code_cb.clone()}
                            value={model.form_data.user_code.clone()}
                        />
                        <label for="device-code" class="input-hint">
                            { "Enter code" }
                        </label>
                    </div>
                </form>
                <br/>
                <button onclick={ctx.props().submit_button_cb.clone()}>
                <p>{ "Continue" }</p>
                </button>
            </>
        }
    }
}