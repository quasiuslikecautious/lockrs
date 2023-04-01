mod styles;
mod components;
mod services;
mod models;
mod views;
mod controllers;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/client/register")]
    ClientRegister,

    #[at("/device")]
    Device,

    #[at("/")]
    Home, 

    #[at("/login")]
    Login,

    #[at("/logout")]
    Logout,

    #[at("/logout/success")]
    LogoutSuccess,

    #[at("/scopes/confirm")]
    ScopeConfirmation,

    #[at("/signup")]
    Signup,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home                 => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::ClientRegister       => html! { <controllers::ClientRegistrationController /> },
        Route::Device               => html! { <controllers::DeviceCodeController /> },
        Route::Login                => html! { <controllers::LoginController /> },
        Route::Logout               => html! { <controllers::LogoutController /> },
        Route::LogoutSuccess        => html! { <controllers::LogoutSuccessController /> },
        Route::ScopeConfirmation    => html! { <controllers::ScopeConfirmationController /> },
        Route::Signup               => html! { <controllers::SignupController /> },
        _ => html! { <h1>{ "Hello Frontend" }</h1> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <components::IdCardContainer>
                <Switch<Route> render={switch} />
            </components::IdCardContainer>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
