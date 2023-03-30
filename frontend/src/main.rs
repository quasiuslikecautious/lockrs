mod styles;
mod components;

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
        Route::ClientRegister       => html! { <components::ClientRegistrationPage /> },
        Route::Device               => html! { <components::DevicePage /> },
        Route::Login                => html! { <components::LoginPage /> },
        Route::Logout               => html! { <components::LogoutPage /> },
        Route::LogoutSuccess        => html! { <components::LogoutSuccessPage /> },
        Route::Signup               => html! { <components::SignupPage /> },
        Route::ScopeConfirmation    => html! { <components::ScopeConfirmationPage /> },
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
