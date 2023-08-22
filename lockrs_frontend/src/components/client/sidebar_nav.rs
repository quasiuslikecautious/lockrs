use leptos::*;
use leptos_router::*;

use crate::components::ui::button::*;
use crate::components::ui::link::*;

#[component]
pub fn SidebarNav(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    items: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    let class = format!(
        "flex space-x-2 lg:flex-col lg:space-x-0 lg:space-y-1 {}",
        class
    );

    view! { cx,
        <nav class=class.clone()>
                <For
                    each=move || items.clone()
                    key=|item| item.1
                    view=move |cx, (title, href)| {
                        let class = format!(
                            "{} {} justify-start",
                            ButtonVariant::Ghost.class(),
                            ButtonSize::Default.class(),
                        );

                        view! { cx,
                                <a
                                    class=move || {
                                        format!(
                                            "{} {}",
                                            class.clone(),
                                            if href == use_router(cx).pathname().get().to_string() {
                                                "bg-muted hover:bg-muted"
                                            } else {
                                                "hover:bg-transparent hover:underline"
                                            }
                                        )
                                    }
                                    href=href
                                >
                                    {title}
                                </a>
                        }
                    }
                />
        </nav>
    }
}
