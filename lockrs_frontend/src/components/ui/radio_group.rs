use leptos::*;

use super::radix_ui::radio_group as Primitive;

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
        <Primitive::RadioGroup
            class=class.clone()
            required=required
            orientation=orientation
            value=value
            default_value=default_value
            set_value=set_value
        >
            {children(cx)}
        </Primitive::RadioGroup>
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
        <Primitive::RadioGroupItem
            class=class.clone()
            value=value
        >
            <Primitive::RadioGroupIndicator class="flex items-center justify-center".to_string() />
        </Primitive::RadioGroupItem>
    }
}
