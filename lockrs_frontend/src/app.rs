use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::*;
use crate::pages::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Layout>
            <Router>
                <Routes>
                    <Route path="" view=  move |cx| view! { cx, <HomePage /> }/>
                    <Route path="/login" view= move |cx| view! { cx, <LoginPage /> }/>
                    <Route path="/register" view= move |cx| view! { cx, <RegisterPage /> }/>
                    // <Route path="/logout" view= move |cx| view! { cx, <LogoutLayout /> }>
                        // <Route path="/success" view= move |cx| view! { cx, <LogoutSuccessPage /> }/>
                        // <Route path="" view=move |cx| view! { cx, <LogoutConfirmationPage /> }/>
                    // </Route>

                    // <Route path="/clients" view= move |cx| view! { cx, <ClientLayout/> }>
                        // <Route path="/register" view= move |cx| view! { cx, <ClientRegisterPage/> }/>
                    // </Route>
                </Routes>
            </Router>
        </Layout>
    }
}
