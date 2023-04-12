mod components;
mod controllers;
mod models;
mod services;
mod styles;
mod views;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/client/:id")]
    ClientDetailsRoute { id: String },

    #[at("/client/register")]
    ClientRegisterRoute,

    #[at("/device")]
    DeviceRoute,

    #[at("/")]
    HomeRoute,

    #[at("/login")]
    LoginRoute,

    #[at("/logout")]
    LogoutRoute,

    #[at("/logout/success")]
    LogoutSuccessRoute,

    #[at("/scopes/confirm")]
    ScopeConfirmationRoute,

    #[at("/signup")]
    SignupRoute,

    #[at("/user/:id")]
    UserDetailsRoute { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomeRoute => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::ClientDetailsRoute { id } => html! { <h1>{ id }</h1> },
        Route::ClientRegisterRoute => html! { <controllers::ClientRegistrationController /> },
        Route::DeviceRoute => html! { <controllers::DeviceCodeController /> },
        Route::LoginRoute => html! { <controllers::LoginController /> },
        Route::LogoutRoute => html! { <controllers::LogoutController /> },
        Route::LogoutSuccessRoute => html! { <controllers::LogoutSuccessController /> },
        Route::ScopeConfirmationRoute => html! { <controllers::ScopeConfirmationController /> },
        Route::SignupRoute => html! { <controllers::SignupController /> },
        Route::UserDetailsRoute { id } => {
            html! { <controllers::UserDetailsController user_id={id} /> }
        }
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
