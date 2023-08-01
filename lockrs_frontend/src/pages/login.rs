use leptos::*;

use crate::components::*;

#[component]
pub fn LoginPage(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, "".to_string());
    let (password, set_password) = create_signal(cx, "".to_string());

    view! { cx,
        <div id="login-page" class="relative text-left h-full">
            <div id="login-title" class="mb-8 py-1 text-center">
                <p class="text-xs uppercase">
                    "Welcome Back"
                </p>
                <h2 class="text-3xl font-bold">
                    "Log into your account"
                </h2>
            </div>

            <form id="login-form" class="mb-2">
                <FormField
                    input_type="text"
                    name="email"
                    label="Email"
                    placeholder="Enter your email"
                    value=email
                    on_change=move |ev| {
                        set_email(event_target_value(&ev));
                    }
                />

                <FormField
                    input_type="password"
                    name="password"
                    label="Password"
                    placeholder="Enter your password"
                    value=password
                    on_change=move |ev| {
                        set_password(event_target_value(&ev));
                    }
                />

                <FilledButton
                    on_click=move |ev| {
                        ev.prevent_default();
                        log::info!("Button clicked");
                    }
                >
                    "Login now"
                </FilledButton>

            </form>

            <div class="text-center">
                <p>
                    "Forgot password?"
                </p>
            </div>

            <div class="absolute bottom-4">
                <p class="text-gray-400">
                    "Don't have an account? " <a href="/register" class="text-white">"Register"</a>
                </p>
            </div>
        </div>
    }
}
