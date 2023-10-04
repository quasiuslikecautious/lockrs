use leptos::*;

use crate::components::client::data_table::*;
use crate::components::ui::separator::*;

#[component]
pub fn ClientAllPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="h-full flex-1 flex-col space-y-8">
            <div>
                <h3 class="text-lg font-medium">All Clients</h3>
                <p class="text-sm text-muted-foreground">
                    Here is a list of all of your clients
                </p>
            </div>
            <Separator />
            <DataTable />
        </div>
    }
}
