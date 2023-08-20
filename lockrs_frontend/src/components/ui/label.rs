use leptos::*;

#[component]
pub fn Label(
    cx: Scope,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] class_signal: Option<Signal<String>>,
    #[prop(default = "".to_string())] html_for: String,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 {}",
        if let Some(class) = class {
            class
        } else {
            String::new()
        }
    );

    view! { cx,
        <label
            class=move || {
                format!(
                    "{} {}",
                    class.clone(),
                    if let Some(class_signal) = class_signal {
                        class_signal()
                    } else {
                        String::new()
                    }
                )
            }
            for=html_for
        >
            {children(cx)}
        </label>
    }
}
