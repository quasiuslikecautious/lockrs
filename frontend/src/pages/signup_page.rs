use yew::prelude::*;

use crate::styles;

#[function_component]
pub fn SignupPage() -> Html {
    html! {
        <div class={ styles::form_styles() }>
            <div class="container" id="form-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Create your account" }</h2>
                <h4>{ "Enter an email and password" }</h4>

                 <form id="device-code-form">
                    <input 
                        type="text" 
                        id="email" 
                        name="email" 
                        placeholder="Enter an email"
                    />
                    <input 
                        type="password"
                        id="password"
                        name="password"
                        placeholder="Enter a password"
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
