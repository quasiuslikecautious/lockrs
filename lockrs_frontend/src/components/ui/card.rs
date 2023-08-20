use leptos::*;

#[component]
pub fn Card(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let default_class = "rounded-lg border bg-card text-card-foreground shadow-sm";
    let end_class = format!("{} {}", default_class, class);

    view! { cx,
        <div class=end_class.clone()>
            {children(cx)}
        </div>
    }
}

#[component]
pub fn CardHeader(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let default_class = "flex flex-col space-y-1.5 p-6";
    let end_class = format!("{} {}", default_class, class);

    view! { cx,
        <div class=end_class.clone()>
            {children(cx)}
        </div>
    }
}

#[component]
pub fn CardTitle(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let default_class = "text-2xl font-semibold leading-none tracking-tight";
    let end_class = format!("{} {}", default_class, class);

    view! { cx,
        <h3 class=end_class.clone()>
            {children(cx)}
        </h3>
    }
}

#[component]
pub fn CardDescription(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let default_class = "text-sm text-muted-foreground";
    let end_class = format!("{} {}", default_class, class);

    view! { cx,
        <p class=end_class.clone()>
            {children(cx)}
        </p>
    }
}

#[component]
pub fn CardContent(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let default_class = "p-6 pt-0";
    let end_class = format!("{} {}", default_class, class);

    view! { cx,
        <div class=end_class.clone()>
            {children(cx)}
        </div>
    }
}

#[component]
pub fn CardFooter(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    children: Children,
) -> impl IntoView {
    let default_class = "flex items-center p-6 pt-0";
    let end_class = format!("{} {}", default_class, class);

    view! { cx,
        <div class=end_class.clone()>
            {children(cx)}
        </div>
    }
}
