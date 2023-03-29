use yew::{function_component, html, Html};

#[function_component(LogoutSuccessPage)]
pub fn logout_success_page() -> Html {
    html! {
        <>
            <h2>{ "Hope to see you soon!" }</h2>
            <h4>{ "You have successfully signed out of your account." }</h4>
            <br/>
            <button>
                <p>{ "Back to login" }</p>
            </button>
        </>
    }
}
