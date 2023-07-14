use leptos::*;
use leptos_router::*;

#[component]
pub fn LogoutLayout(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="logout-page" class="relative text-left h-full">
            <Outlet/>
        </div>
    }
}
