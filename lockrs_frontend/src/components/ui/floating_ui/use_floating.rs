// use leptos::{*, html::ElementDescriptor};
//
// pub struct UseFloatingOptions<W, U, N, El>
// where
//     W: FnOnce(NodeRef<N>, HtmlElement<El>, U),
//     U: FnOnce(),
//     N: ElementDescriptor + 'static,
//     El: ElementDescriptor,
// {
//     cx: Scope,
//     placement: Option<&'static str>,
//     strategy: Option<&'static str>,
//     open: Option<bool>,
//     transform: Option<bool>,
//     reference: NodeRef<N>,
//     floating: N,
//     while_elements_mounted: W,
// }
//
// pub struct UseFloatingReturn {
// }
//
// pub fn use_floating<W, U, N, El>(
//     options: UseFloatingOptions<W, U, N, El>
// ) -> UseFloatingReturn
// where
//     W: FnOnce(NodeRef<N>, HtmlElement<El>, U),
//     U: FnOnce(),
//     N: ElementDescriptor + 'static,
//     El: ElementDescriptor,
// {
//     let cx = options.cx;
//     let placement = Some(options.placement) else { "bottom" };
//     let strategy = Some(options.strategy) else { "absolute" };
//     let open = Some(options.open) else { false };
//     let transform = Some(options.transform) else { true };
//
//     let (_reference, _set_reference) = create_signal(cx, Option::<NodeRef<N>>::None);
//     let (_floating, _set_floating) = create_signal(cx, Option::<N>::None);
//
//     let set_reference = move |node: Option<NodeRef<N>>| {
//         if node != _reference.get() {
//             _set_reference.set(node);
//         }
//     };
//
//     let set_floating = move |node: Option<N>| {
//         if node != _floating() {
//             _set_floating.set(node);
//         }
//     };
//
//     let reference_el =
//         if let Some(val) = options.reference { Some(val) }
//         else { _reference.get_untracked() };
//
//     let floating_el =
//         if let Some(val) = options.floating { Some(val) }
//         else { _floating.with_untracked(cx)() };
//
//
// }
