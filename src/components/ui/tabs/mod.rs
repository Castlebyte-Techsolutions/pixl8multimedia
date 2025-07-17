pub mod types;

use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

#[component]
pub fn Tab(
    #[prop(optional, into)] vertical: MaybeProp<bool>,
    #[prop(optional, into)] gap: MaybeProp<u32>,
    #[prop(optional, into)] extend_class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!(
                "flex {} gap-2 sm:gap-{} {}",
                if vertical.get().unwrap_or(false) { "flex-col" } else { "items-center" },
                gap.get().unwrap_or(3),
                extend_class.get().unwrap_or("".to_string()),
            )
        }>{children()}</div>
    }
}

#[component]
pub fn TabBody(
    #[prop(into)] title: Signal<String>,
    #[prop(into)] link: Signal<String>,
) -> impl IntoView {
    let match_link = RwSignal::new(false);

    Effect::new(move || {
        let location = use_location().pathname.get();
        let match_l = move || {
            location
                .split("/")
                .last()
                .map(|s| s == link.get())
                .unwrap_or(false)
        };
        match_link.set(match_l());
        leptos::logging::log!("{}", match_link.get());
    });

    view! {
        <A
            href=move || format!("/screenplay/{}", link.get())
            attr:class=move || {
                format!(
                    "text-md rounded-md bg-gray-100 px-4 py-2.5 text-center tracking-wide uppercase {}",
                    if match_link.get() {
                        "bg-gray-600 text-stone-100"
                    } else {
                        "group relative inline-flex overflow-hidden "
                    },
                )
            }
        >
            {move || {
                if !match_link.get() {
                    view! {
                        <span class="absolute inset-0 h-full w-0 bg-gray-600 transition-all duration-500 ease-in-out group-hover:w-full"></span>
                        <span class="z-10 transition-colors duration-500 ease-in-out group-hover:text-stone-100">
                            {move || title.get()}
                        </span>
                    }
                        .into_any()
                } else {
                    { move || title.get() }.into_any()
                }
            }}
        </A>
    }
}
