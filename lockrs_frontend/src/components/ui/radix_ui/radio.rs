use leptos::*;
use wasm_bindgen::UnwrapThrowExt;

use crate::components::ui::icons::check_icon::*;

#[derive(Clone, Debug)]
struct RadioContext {
    checked: Signal<bool>,
}

fn get_state(checked: bool) -> &'static str {
    if checked {
        "checked"
    } else {
        "unchecked"
    }
}

#[component]
pub fn Radio(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    #[prop(default = "on")] value: &'static str,
    checked: Signal<bool>,
    #[prop(optional)] on_check: Option<Signal<()>>,
    required: bool,
    children: Children,
) -> impl IntoView {
    let context = RadioContext { checked };
    provide_context(cx, context);

    view! { cx,
        <>
            <button
                class=class.clone()
                role="radio"
                aria-checked=move || { format!("{}", checked()) }
                data-state=move || { get_state(checked()) }
                value=value
                on:click=move |ev| {
                    ev.prevent_default();
                    if !checked() {
                        if let Some(on_check) = on_check {
                            on_check()
                        }
                    }
                }
            >
                {children(cx)}
            </button>

            <BubbleInput
                class="translate-x-full"
                value=value
                checked=checked
                bubbles=true
            />
        </>
    }
}

#[component]
pub fn RadioIndicator(cx: Scope, #[prop(default = String::new())] class: String) -> impl IntoView {
    let context =
        use_context::<RadioContext>(cx).expect_throw("RadioIndicator should be used within Radio");

    view! { cx,
        <Show
            when=context.checked
            fallback=|cx| view! { cx, <div class="hidden" /> }
        >
            <span
                class=class.clone()
                data-state=move || { get_state((context.checked)()) }
            >
                <CheckIcon class="h-3.5 w-3.5 fill-primary" />
            </span>
        </Show>
    }
}

#[component]
pub fn BubbleInput(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    value: &'static str,
    checked: Signal<bool>,
    bubbles: bool,
) -> impl IntoView {
    let class = format!("absolute pointer-events-none opacity-0 m-0 {}", class);

    view! { cx,
        <input
            class=class.clone()
            type="radio"
            value=value
            aria-hidden="true"
            checked=move || { checked() }
            tabIndex=-1
        />
    }
}
