use leptos::{ev, prelude::*};
use leptos_router::components::A;

#[component]
pub fn LinkTagUI<OnCloseAction>(
    #[prop(into)] link: Signal<String>,
    #[prop(into)] title: Signal<String>,
    #[prop(into, optional)] class: Option<MaybeProp<String>>,
    on_close: OnCloseAction,
) -> impl IntoView
where
    OnCloseAction: Fn(ev::MouseEvent) + Sync + 'static + Send + Copy,
{
    let class = class.unwrap_or(
        "uppercase text-md hover:underline font-normal duration-200 ease-in-out transition-all"
            .into(),
    );

    view! {
        <A href=link attr:class=class on:click=on_close>
            {title}
        </A>
    }
}
