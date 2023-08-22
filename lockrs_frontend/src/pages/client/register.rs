use leptos::*;

use crate::components::client::register_form::*;
use crate::components::ui::separator::*;

#[component]
pub fn ClientRegisterPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-medium">Register</h3>
                <p class="text-sm text-muted-foreground">
                    Register a new client for an application
                </p>
            </div>
            <Separator />
            <ClientRegisterForm />
        </div>
    }
}
