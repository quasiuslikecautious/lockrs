use yew::prelude::*;

use crate::styles;

#[function_component]
pub fn LoginPage() -> Html {
    html! {
        <div class={ styles::get_login_styles() }>
            <div class="container" id="login-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Login to your account" }</h2>
                <h4>{ "Enter your email and password" }</h4>

                 <form id="device-code-form">
                    <input 
                        type="text" 
                        id="email" 
                        name="email" 
                        placeholder="Enter email"
                    />
                    <input 
                        type="password"
                        id="password"
                        name="password"
                        placeholder="Enter password"
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
