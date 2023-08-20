use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Link<S>(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    href: S,
    children: Children,
) -> impl IntoView
where
    S: std::fmt::Display + 'static,
{
    let navigate = use_navigate(cx);

    let default_class = "underline underline-offset-4 hover:text-primary";
    let end_class = format!("{} {}", default_class, class);

    view! { cx,
        <a
            class=end_class.clone()
            on:click=move |_| {
                let _ = navigate(href.to_string().as_str(), Default::default());
            }
        >
            {children(cx)}
        </a>
    }
}
