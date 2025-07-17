use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::ui::tabs::{types::TabType, Tab, TabBody};

#[component]
pub fn ScreenplayLayout() -> impl IntoView {
    let link_data = RwSignal::new(screenplay_links());

    view! {
        <section class="min-h-screen flex flex-col gap-3 max-w-7xl mx-auto">
            <div class="my-12"></div>
            <div class="overflow-x-auto scrollbar-hide">
                <Tab gap=5 extend_class="min-w-max justify-center">
                    <For
                        each=move || link_data.get()
                        key=|link_data| link_data.title.clone()
                        let:child
                    >
                        <TabBody title=child.title link=child.link />
                    </For>
                </Tab>
            </div>
            <Outlet />
        </section>
    }
}

fn screenplay_links() -> Vec<TabType> {
    [
        TabType {
            title: "representation".to_string(),
            link: "representation".to_string(),
        },
        TabType {
            title: "query letter".to_string(),
            link: "query-letter".to_string(),
        },
        TabType {
            title: "treatment & logline".to_string(),
            link: "treatment-logline".to_string(),
        },
        TabType {
            title: "trailer".to_string(),
            link: "trailer".to_string(),
        },
        TabType {
            title: "storyboard".to_string(),
            link: "storyboard".to_string(),
        },
        TabType {
            title: "producer's pitch".to_string(),
            link: "producer-pitch".to_string(),
        },
    ]
    .to_vec()
}
