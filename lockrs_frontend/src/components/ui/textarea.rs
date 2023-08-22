use leptos::*;

#[component]
pub fn Textarea(
    cx: Scope,
    #[prop(default = "")] id: &'static str,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(default = "")] placeholder: &'static str,
    #[prop(default = "false")] _disabled: &'static str,
    value: ReadSignal<String>,
) -> impl IntoView {
    let class = format!(
        "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {}",
          if let Some(c) = class {
              c
          } else {
              ""
          }
    );

    view! { cx,
        <textarea
            id=id
            class=class.clone()
            placeholder=placeholder
            value=move ||  { value() }
        />
    }
}
