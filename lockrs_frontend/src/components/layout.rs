use leptos::*;
use leptos_meta::*;

#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <head>
            <Title text="LockRS"/>
        </head>
        <body class="dark bg-background text-foreground h-screen">
            {children(cx)}
        </body>
    }
}
