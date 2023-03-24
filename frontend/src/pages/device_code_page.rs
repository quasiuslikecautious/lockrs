use yew::{function_component, html, Html};

use crate::styles;

#[function_component(DevicePage)]
pub fn device_page() -> Html {
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
                            name="device-code" 
                            minlength="8" 
                            maxlength="8"
                            placeholder=" "
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
