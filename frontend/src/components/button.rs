use leptos::*;

#[component]
pub fn FilledButton<F>(cx: Scope, on_click: F, children: Children) -> impl IntoView
where
    F: FnMut(ev::MouseEvent) -> () + 'static,
{
    view! { cx,
        <button
            class="rounded-xl w-full px-6 py-4 mt-4 font-bold text-center bg-gradient-to-r from-teal-500 via-emerald-500 to-green-500"
            on:click=on_click
        >
            {children(cx)}
        </button>
    }
}

#[component]
pub fn UnfilledButton<F>(cx: Scope, on_click: F, children: Children) -> impl IntoView
where
    F: FnMut(ev::MouseEvent) -> () + 'static,
{
    view! { cx,
        <div
            class="rounded-xl w-full p-px mt-4 bg-gradient-to-r from-teal-500 via-emerald-500 to-green-500"
        >
            <button
                class="rounded-xl w-full px-6 py-4 font-bold text-center bg-gray-800"
                on:click=on_click
            >
                {children(cx)}
            </button>
        </div>

    }
}
