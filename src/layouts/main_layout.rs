use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::sections::{FooterSection, HeaderSection};

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <HeaderSection />
        <main class="overflow-y-scroll grow">
            <Outlet />
        </main>
        <FooterSection />
    }
}
