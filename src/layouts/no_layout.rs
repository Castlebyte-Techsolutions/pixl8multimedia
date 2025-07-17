use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::Outlet;

#[component]
pub fn NoLayout() -> impl IntoView {
    view! {
        <Title text="Services" />

        <section class="min-h-screen">
            <Outlet />
        </section>
    }
}
