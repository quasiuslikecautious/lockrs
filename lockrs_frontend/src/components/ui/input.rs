use leptos::*;

#[component]
pub fn Input(
    cx: Scope,
    #[prop(default = "")] id: &'static str,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(default = "")] placeholder: &'static str,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(default = "off")] autocapitalize: &'static str,
    #[prop(default = "off")] autocomplete: &'static str,
    #[prop(default = "off")] autocorrect: &'static str,
    #[prop(default = "false")] _disabled: &'static str,
    value: ReadSignal<String>,
) -> impl IntoView {
    let class = format!(
        "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 {}",
        if let Some(c) = class {
            c
        } else {
            ""
        }
    );

    view! { cx,
        <input
            id=id
            class=class.clone()
            placeholder=placeholder
            type=input_type
            autocapitalize=autocapitalize
            autocomplete=autocomplete
            autocorrect=autocorrect
            value=move || { value() }
        />
    }
}
