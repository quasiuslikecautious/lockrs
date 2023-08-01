use leptos::*;

use crate::components::*;

#[component]
pub fn RegisterPage(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, "".to_string());
    let (password, set_password) = create_signal(cx, "".to_string());

    view! { cx,
        <div id="register-page" class="relative text-left h-full">
            <div id="register-title" class="mb-8 py-1 text-center">
                <p class="text-xs uppercase">
                    "Nice to meet you"
                </p>
                <h2 class="text-3xl font-bold">
                    "Create your account"
                </h2>
            </div>

            <form id="register-form" class="mb-2">
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
                    placeholder="Enter a secure password"
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
                    "Register now"
                </FilledButton>


            </form>

            <div class="absolute bottom-4">
                <p class="text-gray-400">
                    "Already have an account? " <a href="/login" class="text-white">"Login"</a>
                </p>
            </div>
        </div>
    }
}
