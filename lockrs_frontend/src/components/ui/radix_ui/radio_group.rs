use leptos::*;
use wasm_bindgen::UnwrapThrowExt;

use super::radio::*;

#[derive(Clone, Debug)]
struct RadioGroupContext {
    pub required: bool,
    pub value: ReadSignal<String>,
    pub set_value: WriteSignal<String>,
}

#[component]
pub fn RadioGroup(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    #[prop(default = true)] required: bool,
    #[prop(default = "horizontal")] orientation: &'static str,
    value: ReadSignal<String>,
    default_value: Option<&'static str>,
    set_value: WriteSignal<String>,
    children: Children,
) -> impl IntoView {
    let context = RadioGroupContext {
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
pub fn RadioGroupItem(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    value: &'static str,
    children: Children,
) -> impl IntoView {
    let context = use_context::<RadioGroupContext>(cx)
        .expect_throw("RadioGroupItem should be used within RadioGroup");

    let checked = Signal::derive(cx, move || (context.value)() == value);
    let on_check = Signal::derive(cx, move || {
        context
            .set_value
            .update(|v: &mut String| *v = value.to_string());
    });

    view! { cx,
        <Radio
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
        </Radio>
    }
}

#[component]
pub fn RadioGroupIndicator(
    cx: Scope,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! { cx,
        <RadioIndicator class=class.clone() />
    }
}
