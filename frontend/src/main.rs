mod styles;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    
    #[at("/login")]
    Login,

    #[at("/device")]
    Device,

    #[at("/client/register")]
    ClientRegister,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home             => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::Login            => html! { <pages::LoginPage /> },
        Route::Device           => html! { <pages::DevicePage /> },
        Route::ClientRegister   => html! { <pages::ClientRegistrationPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
