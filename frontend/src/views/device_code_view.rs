use yew::prelude::*;

use crate::models::DeviceCodeModel;

#[derive(Clone, PartialEq)]
pub struct DeviceCodeFormCallbacks {
    pub on_submit: Callback<MouseEvent>,
    pub on_user_code_change: Callback<Event>
}

#[derive(Clone, Properties, PartialEq)]
pub struct DeviceCodeViewProps {
    pub model: DeviceCodeModel,
    pub form_callbacks: DeviceCodeFormCallbacks,
}

pub struct DeviceCodeView;

impl Component for DeviceCodeView {
    type Message = ();
    type Properties = DeviceCodeViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                            onchange={ctx.props().form_callbacks.on_user_code_change.clone()}
                            value={ctx.props().model.user_code.clone()}
                        />
                        <label for="device-code" class="input-hint">
                            { "Enter code" }
                        </label>
                    </div>
                </form>
                <br/>
                <button onclick={ctx.props().form_callbacks.on_submit.clone()}>
                <p>{ "Continue" }</p>
                </button>
            </>
        }
    }
}
