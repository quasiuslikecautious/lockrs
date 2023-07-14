use leptos::*;

#[component]
pub fn LogoutSuccessPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="logout-success-page" class="relative text-left h-full">
            <div>
                <div id="logout-success-title" class="mb-8 py-1 text-center">
                    <h2 class="text-3xl font-bold">
                        "You have been successfully logged out"
                    </h2>
                </div>
            </div>

        </div>
    }
}
