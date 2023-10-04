// use leptos::{*, html::{Div, Span}};
// use wasm_bindgen::UnwrapThrowExt;
//
// #[derive(Clone)]
// struct PopperContext<F>
// where
//     F: FnOnce(Measurable) + Clone,
// {
//     pub anchor: Measurable,
//     pub on_anchor_change: F,
// }
//
// #[component]
// pub fn Popper(
//     cx: Scope,
//     children: Children,
// ) -> impl IntoView {
//     let (anchor, set_anchor) = create_signal(cx, Measurable::new());
//     let context = PopperContext { anchor, on_anchor_change = set_anchor };
//
//     view! { cx,
//         <>
//             {children(cx)}
//         </>
//     }
// }
//
// #[component]
// pub fn PopperAnchor(
//     cx: Scope,
//     children: Children,
// ) -> impl IntoView {
//     let context = use_context::<PopperContext>(cx).expect_throw("PopperAnchor should be used within a Popper");
//
//     view! { cx,
//         <div>
//             {children(cx)}
//         </div>
//     }
// }
//
// #[component]
// pub fn PopperContent<F>(
//     cx: Scope,
//     #[prop(default = "bottom")] side: &'static str,
//     #[prop(default = 0)] side_offset: i32,
//     #[prop(default = "center")] align: &'static str,
//     #[prop(default = 0)] arrow_padding: i32,
//     #[prop(default = true)] avoid_collisions: bool,
//     #[prop(default = Vec::<i32>::new())] collision_boundary: Vec<i32>,
//     #[prop(default = 0)] collision_padding: i32,
//     #[prop(default = "sticky")] sticky: &'static str,
//     #[prop(default = false)] hide_when_detached: bool,
//     #[prop(default = "optimized")] update_position_strategy: &'static str,
//     on_placed: F,
//     children: Children,
// ) -> impl IntoView
// where
//     F: FnOnce(),
// {
//     let context = use_context::<PopperContext>(cx).expect_throw("PopperContent should be used within a Popper");
//
//     let (content, set_content) = create_signal(cx, Option<Div>);
//     let (arrow, set_arrow) = create_signal(cx, Option<Span>);
//
//     let arrow_width = 4;
//     let arrow_height = 4;
//
//     let desired_placment = side + (if align != "center" { format!("-{}", align) } else { String::new() });
//
//     let has_explicit_boundaries = collision_boundary.len() > 0;
//
//
//
//     view! { cx,
//         <>
//         </>
//     }
// }
