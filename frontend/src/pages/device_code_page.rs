use yew::prelude::*;

use crate::styles;

#[function_component]
pub fn DevicePage() -> Html {
    html! {
        <div class={ styles::get_device_styles() } id="device-code-page">
            <div class="container" id="device-code-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Connect a device" }</h2>
                <h4>{ "Enter the code displayed on your device" }</h4>

                <form id="device-code-form">
                    <input 
                        type="text" 
                        id="device-code-input" 
                        name="device-code" 
                        placeholder="Enter code"
                    />
                </form>
                <br/>
                <button>
                   <p>{ "Continue" }</p>
                </button>
            </div>
        </div>
    }
}
