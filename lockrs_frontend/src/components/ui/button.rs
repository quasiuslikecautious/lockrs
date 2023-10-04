use leptos::*;

use crate::utils::tailwind_merge::cn;

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
    #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
    children: Children,
) -> impl IntoView {
    let variant_class = format!("{} {}", variant.class(), size.class());
    let additional_class = class.unwrap_or(String::new());

    let disabled = Signal::derive(cx, move || {
        if let Some(disabled) = disabled {
            disabled.get()
        } else {
            false
        }
    });

    let (class, set_class) = create_signal(cx, String::new());
    let client_class = create_effect(cx, move |_| {
        let resolved_class = cn(variant_class.clone().as_str(), additional_class.clone().as_str());
        set_class(resolved_class);
    });

    view! { cx,
        <button
            class=move || class()
            disabled=disabled
        >
            {children(cx)}
        </button>
    }
}
