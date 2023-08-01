use leptos::*;

use crate::components::*;

#[component]
pub fn LogoutConfirmationPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="logout-confirmation-page" class="relative text-left h-full">
            <div id="logout-title" class="mb-8 py-1 text-center">
                <p class="text-xs uppercase">
                    "Miss you already"
                </p>
                <h2 class="text-3xl font-bold">
                    "Are you sure you want to log out?"
                </h2>
            </div>

            <form action="/logout/success">
                <FilledButton
                    on_click=move |_ev| {
                        log::info!("Button clicked");
                    }
            >
                "Log out now"
            </FilledButton>
            </form>

            <UnfilledButton
                on_click=move |_ev| {
                    log::info!("Button clicked");
                }
            >
                "Never mind"
            </UnfilledButton>
        </div>
    }
}
