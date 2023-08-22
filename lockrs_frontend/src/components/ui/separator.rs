use leptos::*;

pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Horizontal => "h-[1px] w-full",
            Self::Vertical => "h-full w-[1px]",
        }
    }

    pub fn data_orientation(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    pub fn aria_orientation(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            _ => "",
        }
    }
}

#[component]
pub fn Separator(
    cx: Scope,
    #[prop(default = "")] class: &'static str,
    #[prop(default = SeparatorOrientation::Horizontal)] orientation: SeparatorOrientation,
    #[prop(default = true)] decorative: bool,
) -> impl IntoView {
    let class = format!("shrink-0 bg-border {} {}", orientation.class(), class,);

    let role = if decorative { "none" } else { "separator" };

    view! { cx,
        <div
            class=class.clone()
            data-orientation=orientation.data_orientation()
            aria_orientation=orientation.aria_orientation()
            role=role
        />
    }
}
