use leptos::{ev, html::Div, prelude::*};
use leptos_router::{components::A, hooks::use_location};
use leptos_use::{on_click_outside, use_window_scroll, use_window_size, UseWindowSizeReturn};
use phosphor_leptos::{Icon, LIST};

#[component]
pub fn HeaderSection() -> impl IntoView {
    let (_, y) = use_window_scroll();
    let scrolled = move || y() > 20.0;
    let home_route = RwSignal::new(false);
    let is_show = RwSignal::new(false);
    let UseWindowSizeReturn { width, height: _ } = use_window_size();

    Effect::new(move || {
        let location = use_location().pathname.get();
        home_route.set(location.eq("/"));
    });

    let links_data = RwSignal::new([
        ("/", "home"),
        ("https://hunt-publishing.pixl8media.com", "hunt publishing"),
        ("https://sab-marketing.pixl8media.com", "sab marketing"),
        ("/screenplay", "screenplay adaptation"),
        ("/service", "upload materials"),
        ("/about-us", "about us"),
    ]);

    let toggle_show = move |_| is_show.set(false);

    let links_view = move || {
        links_data
            .get()
            .into_iter()
            .map(|(link_href, link_title)| {
                view! { <HeaderLink title=link_title link=link_href /> }.into_any()
            })
            .collect_view()
    };

    let sidebar_node = NodeRef::<Div>::new();
    let _ = on_click_outside(sidebar_node, move |_| is_show.set(false));

    view! {
        <header class="fixed top-0 z-[999] w-full overflow-hidden">
            <span class=move || {
                format!(
                    "absolute left-1/2 top-0 -translate-x-1/2 h-full bg-indigoblue-400/50 bg-clip-padding backdrop-filter backdrop-blur backdrop-saturate-100 backdrop-contrast-100 bg-blend-overlay transition-all duration-300 ease-linear {}",
                    if scrolled() { "w-full" } else { "w-0" },
                )
            }></span>

            <nav class="container relative z-10 mx-auto my-5 flex items-stretch gap-5 px-5">
                <A href="/" attr:class="flex relative overflow-hidden">
                    <img
                        src=move || {
                            format!(
                                "/images/logos/pixel8media-logo-edited{}.png",
                                if !scrolled() {
                                    if home_route.get() { "" } else { "-black" }
                                } else {
                                    ""
                                },
                            )
                        }
                        alt=""
                        class=move || {
                            format!(
                                "h-12 {}",
                                if scrolled() { "fade-in" } else { "scale-in-center" },
                            )
                        }
                    />
                </A>
                {move || {
                    if width.get() >= 1650.0 {
                        view! {
                            <div class=move || {
                                format!(
                                    "flex-1 hidden xl:flex justify-center gap-5 items-center px-5 py-2.5 font-bold tracking-wider text-gray-50 {}",
                                    if scrolled() {
                                        "transition-all duration-300 ease-linear"
                                    } else {
                                        "transition-all duration-300 ease-linear rounded-full bg-indigoblue-400/50 bg-clip-padding backdrop-filter backdrop-blur backdrop-saturate-100 backdrop-contrast-100 bg-blend-overlay shadow-lg"
                                    },
                                )
                            }>{links_view()}</div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="mx-auto"></div>
                            <button
                                on:click=move |_| is_show.set(!is_show.get())
                                class="bg-gray-200 cursor-pointer hover:bg-gray-500 hover:text-gray-50 rounded-md group overflow-hidden relative inline-flex p-1.5 items-center"
                            >
                                <Icon icon=LIST attr:class="h-6 w-10" />
                            </button>
                        }
                            .into_any()
                    }
                }}
                <A
                    href="/contact-us"
                    attr:class=move || {
                        format!(
                            "group relative hidden xl:inline-flex items-center cursor-pointer overflow-hidden px-4 py-2.5 shadow-lg font-playfair text-stone-50 rounded-full {}",
                            if scrolled() {
                                "duration-300 transition-all ease-in-out bg-gray-400/50 text-stone-50"
                            } else {
                                "transition-all duration-300 ease-in-out bg-indigoblue-400/50 bg-clip-padding backdrop-filter backdrop-blur backdrop-saturate-100 backdrop-contrast-100 bg-blend-overlay"
                            },
                        )
                    }
                >
                    <span class="absolute left-1/2 top-0 -translate-x-1/2 z-0 h-full w-0 bg-gray-800 transition-all duration-300 ease-linear group-hover:w-full"></span>
                    <span class="z-10 uppercase font-bold tracking-wider transition-colors duration-300 ease-linear group-hover:text-white">
                        "Contact Us"
                    </span>
                </A>
            </nav>

        </header>

        <Show when=move || is_show.get() fallback=|| ()>
            <SideBarMenu node_ref=sidebar_node on_close=toggle_show />
        </Show>
    }
}

#[component]
fn SideBarMenu<OnCloseAction>(on_close: OnCloseAction, node_ref: NodeRef<Div>) -> impl IntoView
where
    OnCloseAction: Fn(ev::MouseEvent) + 'static + Send + Sync + Copy,
{
    use leptos_icons::Icon;

    view! {
        <div
            node_ref=node_ref
            class="z-[999] fixed top-0 right-0 w-full h-lvh sm:w-96 bg-indigoblue-400"
        >
            <div class="h-full w-full px-3 py-5 flex flex-col gap-2 text-white">
                <div class="flex items-center justify-between">
                    <A href="/" on:click=on_close attr:class="flex relative overflow-hidden">
                        <img src="/images/logos/pixel8media-logo-edited.png" alt="" class="h-12" />
                    </A>
                    <button
                        on:click=on_close
                        class="bg-gray-200 cursor-pointer hover:bg-gray-500 text-stone-900 hover:text-gray-50 rounded-md group overflow-hidden relative inline-flex p-1.5 items-center"
                    >
                        <Icon icon=icondata::CgClose attr:class="h-6 w-10" />
                    </button>
                </div>

                <A
                    href="/"
                    on:click=on_close
                    attr:class="text-lg font-bold tracking-wider font-playfair text-center py-2 w-full uppercase"
                >
                    "Home"
                </A>
                <A
                    on:click=on_close
                    href="https://hunt-publishing.pixl8media.com"
                    attr:class="text-lg font-bold tracking-wider font-playfair text-center py-2 w-full uppercase"
                >
                    "hunt publishing"
                </A>
                <A
                    on:click=on_close
                    href="https://sab-marketing.pixl8media.com"
                    attr:class="text-lg font-bold tracking-wider font-playfair text-center py-2 w-full uppercase"
                >
                    "sab marketing"
                </A>
                <A
                    on:click=on_close
                    href="/screenplay"
                    attr:class="text-lg font-bold tracking-wider font-playfair text-center py-2 w-full uppercase"
                >
                    "screenplay adaptation"
                </A>
                <A
                    on:click=on_close
                    href="/service"
                    attr:class="text-lg font-bold tracking-wider font-playfair text-center py-2 w-full uppercase"
                >
                    "upload materials"
                </A>
                <A
                    on:click=on_close
                    href="/about-us"
                    attr:class="text-lg font-bold tracking-wider font-playfair text-center py-2 w-full uppercase"
                >
                    "about us"
                </A>

                <div class="mt-auto flex flex-col gap-3 items-center justify-center w-full">
                    <A
                        on:click=on_close
                        href="/contact-us"
                        attr:class="text-lg font-bold tracking-wider font-playfair text-center py-2 uppercase rounded-full"
                    >
                        "contact us"
                    </A>

                    <div class="flex items-center justify-center gap-3">
                        <A href="https://facebook.com/pxl8media" target="_blank" attr:class="">
                            <Icon icon=icondata::ImFacebook attr:class="h-5 w-5" />
                        </A>
                        <A href="https://facebook.com/pxl8media" target="_blank" attr:class="">
                            <Icon icon=icondata::SiMessenger attr:class="h-5 w-5" />
                        </A>
                        <A
                            href="https://www.youtube.com/@LunaGlobalMedia"
                            target="_blank"
                            attr:class=""
                        >
                            <Icon icon=icondata::ImYoutube attr:class="h-5 w-5" />
                        </A>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn HeaderLink(
    #[prop(into)] title: Signal<String>,
    #[prop(into)] link: Signal<String>,
) -> impl IntoView {
    view! {
        <A
            href=move || link.get()
            attr:class="relative inline-flex items-center overflow-hidden px-3 py-2 group uppercase font-playfair"
        >
            <span class="absolute inset-0 left-1/2 -translate-x-1/2 bg-indigoblue-600 w-0 group-hover:w-full transition-all duration-300 ease-in-out"></span>
            <span class="self-center z-10 font-bold tracking-wider">{move || title.get()}</span>
        </A>
    }
}
