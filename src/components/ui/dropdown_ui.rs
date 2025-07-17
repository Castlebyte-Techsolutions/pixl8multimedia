use leptos::{ev, html::Button, prelude::*};
use leptos_use::on_click_outside;

use crate::{components::ui::LinkTagUI, utils::types::LinkTagType};

#[component]
pub fn DropdownUI<OnCloseAction>(
    #[prop(into)] title: Signal<String>,
    #[prop(into)] sub_links: Signal<Vec<LinkTagType>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    on_close: OnCloseAction,
) -> impl IntoView
where
    OnCloseAction: Fn(ev::MouseEvent) + Sync + Send + 'static + Copy,
{
    let show_sub_links = RwSignal::new(false);
    let btn_ref = NodeRef::<Button>::new();

    let _ = on_click_outside(btn_ref, move |_| show_sub_links.set(false));

    view! {
        <div class=format!("flex relative {}", class.get().unwrap_or_default())>
            <button
                class="uppercase"
                node_ref=btn_ref
                on:click=move |_| show_sub_links.set(!show_sub_links.get())
            >
                {title}
            </button>
            {move || {
                if show_sub_links.get() {
                    view! {
                        <div class="absolute top-8 left-0 bg-gray-100 z-50 text-black">

                            <div class="flex flex-col gap-3 px-4 py-2.5 rounded-md w-[250px]">
                                <For
                                    each=move || sub_links.get()
                                    key=|sub_links| sub_links.title.clone()
                                    let:sub_link
                                >
                                    <LinkTagUI
                                        title=sub_link.title.clone()
                                        link=sub_link.link.clone()
                                        on_close
                                    />
                                </For>
                            </div>
                        </div>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}
