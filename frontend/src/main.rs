mod styles;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    
    #[at("/signup")]
    Signup,

    #[at("/login")]
    Login,

    #[at("/logout")]
    Logout,

    #[at("/logout/success")]
    LogoutSuccess,

    #[at("/device")]
    Device,

    #[at("/client/register")]
    ClientRegister,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home             => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::Signup           => html! { <components::SignupPage /> },
        Route::Login            => html! { <components::LoginPage /> },
        Route::Logout           => html! { <components::LogoutPage /> },
        Route::LogoutSuccess    => html! { <components::LogoutSuccessPage /> },
        Route::Device           => html! { <components::DevicePage /> },
        Route::ClientRegister   => html! { <components::ClientRegistrationPage /> },
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
