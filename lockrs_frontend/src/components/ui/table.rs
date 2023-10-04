use leptos::*;

#[component]
pub fn Table(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!("w-full caption-bottom text-sm {}", class,);

    view! { cx,
        <div class="w-full overflow-auto">
            <table class=class.clone()>
                {children(cx)}
            </table>
        </div>
    }
}

#[component]
pub fn TableHeader(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!("[&_tr]:border-b {}", class,);

    view! { cx,
        <thead class=class.clone()>
            {children(cx)}
        </thead>
    }
}

#[component]
pub fn TableBody(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!("[&_tr:last-child]:border-0 {}", class,);

    view! { cx,
        <tbody class=class.clone()>
            {children(cx)}
        </tbody>
    }
}

#[component]
pub fn TableFooter(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!("bg-primary font-medium text-primary-foreground {}", class,);

    view! { cx,
        <tfoot class=class.clone()>
            {children(cx)}
        </tfoot>
    }
}

#[component]
pub fn TableRow(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    #[prop(default = "none")] data_state: &'static str,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted {}",
        class,
    );

    view! { cx,
        <tr
            class=class.clone()
            data-state=data_state
        >
            {children(cx)}
        </tr>
    }
}

#[component]
pub fn TableHead(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {}",
        class,
    );

    view! { cx,
        <th class=class.clone()>
            {children(cx)}
        </th>
    }
}

#[component]
pub fn TableCell(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {}",
        class,
    );

    view! { cx,
        <td class=class.clone()>
            {children(cx)}
        </td>
    }
}

#[component]
pub fn TableCaption(
    cx: Scope,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!("mt-4 text-sm text-muted-foreground {}", class,);

    view! { cx,
        <caption class=class>
            {children(cx)}
        </caption>
    }
}
