use leptos::*;

#[component]
pub fn FormField<F>(
    cx: Scope,
    input_type: &'static str,
    name: &'static str,
    label: &'static str,
    placeholder: &'static str,
    value: ReadSignal<String>,
    on_change: F,
) -> impl IntoView
where
    F: FnMut(ev::Event) -> () + 'static,
{
    view! { cx,
        <div class="mb-4">
            <label for=name>
                <p class="mb-1 text-base">
                    {label}
                </p>
            </label>

            <input
                type=input_type
                id=name
                name=name
                class="custom-input w-full"
                placeholder=placeholder
                on:change=on_change
                prop:value=value
            />

            <div class="ml-auto mr-2">
                <p class="text-red-500 text-xs font-bold">
                    ""
                </p>
            </div>
        </div>
    }
}
