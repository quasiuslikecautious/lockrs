use leptos::*;

use crate::components::*;

#[component]
pub fn ClientRegisterPage(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "".to_string());
    let (description, set_description) = create_signal(cx, "".to_string());
    let (app_type, set_type) = create_signal(cx, "".to_string());
    let (homepage, set_homepage) = create_signal(cx, "".to_string());
    let (redirect, set_redirect) = create_signal(cx, "".to_string());

    view! { cx,
        <div id="login-page" class="relative text-left h-full">
            <div id="login-title" class="mb-8 py-1 text-center">
                <p class="text-xs uppercase">
                    "Unleashing Limitless Possibilities"
                </p>
                <h2 class="text-3xl font-bold">
                    "Register a new client"
                </h2>
            </div>

            <form id="client-register-form" class="mb-2">
                <FormField
                    input_type="text"
                    name="name"
                    label="Name"
                    placeholder="Enter the name of your app"
                    value=name
                    on_change=move |ev| {
                        set_name(event_target_value(&ev));
                    }
                />

                <FormField
                    input_type="text"
                    name="description"
                    label="Description"
                    placeholder="Enter a short description of your app"
                    value=description
                    on_change=move |ev| {
                        set_description(event_target_value(&ev));
                    }
                />

                <DropDownSelect
                    name="type"
                    label="Type"
                    placeholder=""
                    value=app_type
                    on_change=move |ev| {
                        set_type(event_target_value(&ev));
                    }
                >
                    <option value="public">"Public"</option>
                    <option value="private">"Private"</option>
                </DropDownSelect>

                <FormField
                    input_type="text"
                    name="homepage"
                    label="Homepage URL"
                    placeholder="Enter the URL for your app"
                    value=homepage
                    on_change=move |ev| {
                        set_description(event_target_value(&ev));
                    }
                />

                <FormField
                    input_type="text"
                    name="redirect"
                    label="Redirect URL"
                    placeholder="Enter the OAuth2 redirect URL for your app"
                    value=redirect
                    on_change=move |ev| {
                        set_description(event_target_value(&ev));
                    }
                />

                <FilledButton
                    on_click=move |ev| {
                        ev.prevent_default();
                        log::info!("Button clicked");
                    }
                >
                    "Register your client"
                </FilledButton>
            </form>
        </div>
    }
}
