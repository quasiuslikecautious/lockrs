use leptos::*;
use leptos_meta::*;

#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <head>
            <Title text="LockRS"/>
        </head>
        <body class="p-4 pt-16 bg-gradient-to-t from-gray-800 to-gray-700 text-white h-screen">
            <div id="content" class="h-full">
                {children(cx)}
            </div>
        </body>
    }
}
