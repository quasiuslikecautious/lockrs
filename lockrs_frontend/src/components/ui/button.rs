use leptos::*;

pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn class(&self) -> String {
        format!("inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 {}",
        match self {
            Self::Default => "bg-primary text-primary-foreground shadow hover:bg-primary/90",
            Self::Destructive => "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90",
            Self::Outline => "border border-input bg-transparent shadow-sm hover:bg-accent hover:text-accent-foreground",
            Self::Secondary => "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",
            Self::Ghost => "hover:bg-accent hover:text-accent-foreground",
            Self::Link => "text-primary underline-offset-4 hover:underline",
        })
    }
}

pub enum ButtonSize {
    Default,
    Small,
    Large,
    Icon,
    NoPad,
}

impl ButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "h-9 px-4 py-2",
            Self::Small => "h-8 rounded-md px-3 text-xs",
            Self::Large => "h-10 rounded-md px-8",
            Self::Icon => "h-9 w-9",
            Self::NoPad => "p-none",
        }
    }
}

#[component]
pub fn Button(
    cx: Scope,
    #[prop(optional)] class: Option<String>,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "{} {} {}",
        variant.class(),
        size.class(),
        if let Some(c) = class {
            c
        } else {
            String::new()
        }
    );

    view! { cx,
        <button
            class={class.clone()}
        >
            {children(cx)}
        </button>
    }
}
