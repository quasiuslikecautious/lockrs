use yew::prelude::*;

use crate::styles;

#[function_component]
pub fn ClientRegistrationPage() -> Html {
    html! {
        <div class={ styles::form_styles() } id="client-registration-page">
            <div class="container" id="form-container">
                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />
                <h2>{ "Register a client" }</h2>
                <h4>{ "Fill out the required fields to register a client" }</h4>

                <form id="form-container">
                    <input 
                        type="text" 
                        id="app-name" 
                        name="app-name" 
                        placeholder="Application name"
                    />
                    <input 
                        type="text" 
                        id="app-homepage" 
                        name="app-homepage" 
                        placeholder="Homepage URL"
                    />
                    <input 
                        type="text" 
                        id="app-description" 
                        name="app-description" 
                        placeholder="Application Description"
                    />
                    <input 
                        type="text" 
                        id="callback-url" 
                        name="callback-url" 
                        placeholder="Authorization callback URL"
                    />
                </form>
                <br/>
                <button>
                   <p>{ "Register" }</p>
                </button>


            </div>
        </div>
    }
}
