use leptos::*;

#[component]
pub fn DropDownSelect<F>(
    cx: Scope,
    name: &'static str,
    label: &'static str,
    placeholder: &'static str,
    value: ReadSignal<String>,
    on_change: F,
    children: Children,
) -> impl IntoView
where
    F: FnMut(ev::Event) + 'static,
{
    view! { cx,
        <div class="mb-4">
            <label for=name>
                <p class="mb-1 text-base">
                    {label}
                </p>
            </label>

            <select
                id=name
                class="custom-input w-full"
                name=name
                on:change=on_change
                value=value
            >
                <option
                    value=placeholder
                    disabled=true
                    selected=true
                    hidden=true
                />

                {children(cx)}
            </select>
        </div>
    }
}
