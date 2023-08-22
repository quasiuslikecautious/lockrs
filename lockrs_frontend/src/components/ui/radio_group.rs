use leptos::*;
use wasm_bindgen::UnwrapThrowExt;

use crate::components::ui::icons::check_icon::*;

#[component]
pub fn RadioGroup(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    #[prop(default = true)] required: bool,
    #[prop(default = "horizontal")] orientation: &'static str,
    value: ReadSignal<String>,
    #[prop(optional)] default_value: Option<&'static str>,
    set_value: WriteSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = format!("grid gap-2 {}", class);

    if let Some(default_value) = default_value {
        set_value(default_value.to_string());
    }

    view! { cx,
        <RadioGroupPrimitive
            class=class.clone()
            required=required
            orientation=orientation
            value=value
            default_value=default_value
            set_value=set_value
        >
            {children(cx)}
        </RadioGroupPrimitive>
    }
}

#[component]
pub fn RadioGroupItem(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    value: &'static str,
) -> impl IntoView {
    let class = format!(
        "aspect-square h-4 w-4 rounded-full border border-primary text-primary shadow focus:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 {}",
        class
    );

    view! { cx,
        <RadioGroupItemPrimitive
            class=class.clone()
            value=value
        >
            <RadioGroupIndicatorPrimitive class="flex items-center justify-center".to_string() />
        </RadioGroupItemPrimitive>
    }
}

#[derive(Clone, Debug)]
struct RadioGroupPrimitiveContext {
    pub required: bool,
    pub value: ReadSignal<String>,
    pub set_value: WriteSignal<String>,
}

#[component]
fn RadioGroupPrimitive(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    #[prop(default = true)] required: bool,
    #[prop(default = "horizontal")] orientation: &'static str,
    value: ReadSignal<String>,
    default_value: Option<&'static str>,
    set_value: WriteSignal<String>,
    children: Children,
) -> impl IntoView {
    let context = RadioGroupPrimitiveContext {
        required,
        value,
        set_value,
    };
    provide_context(cx, context);

    view! { cx,
        <div
            class=class.clone()
            role="radiogroup"
            aria-required=required
            aria-orientation=orientation
            class=class.clone()
        >
            {children(cx)}
        </div>
    }
}

#[component]
fn RadioGroupItemPrimitive(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    value: &'static str,
    children: Children,
) -> impl IntoView {
    let context = use_context::<RadioGroupPrimitiveContext>(cx)
        .expect_throw("RadioGroupItem should be used within RadioGroup");

    let checked = Signal::derive(cx, move || (context.value)() == value);
    let on_check = Signal::derive(cx, move || {
        context
            .set_value
            .update(|v: &mut String| *v = value.to_string());
    });

    view! { cx,
        <RadioPrimitive
            class=class.clone()
            value=value
            checked=checked
            on_check=on_check
            required=context.required
            on:keydown=|ev| {
                ev.prevent_default();
            }
        >
            {children(cx)}
        </RadioPrimitive>
    }
}

#[component]
fn RadioGroupIndicatorPrimitive(
    cx: Scope,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! { cx,
        <RadioIndicatorPrimitive class=class.clone() />
    }
}

// -------- RADIO --------
#[derive(Clone, Debug)]
struct RadioPrimitiveContext {
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
fn RadioPrimitive(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    #[prop(default = "on")] value: &'static str,
    checked: Signal<bool>,
    #[prop(optional)] on_check: Option<Signal<()>>,
    required: bool,
    children: Children,
) -> impl IntoView {
    let context = RadioPrimitiveContext { checked };
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
fn RadioIndicatorPrimitive(
    cx: Scope,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let context = use_context::<RadioPrimitiveContext>(cx)
        .expect_throw("RadioIndicator should be used within Radio");

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
fn BubbleInput(
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
