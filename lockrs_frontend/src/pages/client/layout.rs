use leptos::*;
use leptos_router::*;

use crate::components::client::sidebar_nav::*;
use crate::components::ui::separator::*;

#[component]
pub fn ClientLayout(cx: Scope) -> impl IntoView {
    let routes: Vec<(&'static str, &'static str)> =
        vec![("New", "/clients/register"), ("All", "/clients")];

    view! { cx,
        <div id="client-page" class="space-y-6 p-10 pb-16">
            <div class="space-y-0.5">
                <h2 class="text-2xl font-bold tracking-tight">Client</h2>
                <p class="text-muted-foreground">Manage your clients and related settings</p>
            </div>
            <Separator class="my-6" />
            <div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
                <aside class="-mx-4 lg:w-1/5">
                    <SidebarNav items=routes />
                </aside>
                <div class="flex-1 lg:max-w-2xl">
                    <Outlet />
                </div>
            </div>
        </div>
    }
}
