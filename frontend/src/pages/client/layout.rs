use leptos::*;
use leptos_router::*;

#[component]
pub fn ClientLayout(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="client-page" class="relative text-left h-full">
            <Outlet/>
        </div>
    }
}
