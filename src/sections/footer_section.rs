use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

use crate::{components::ui::LinkTagUI, utils::datas};

#[component]
pub fn FooterSection() -> impl IntoView {
    let links = RwSignal::new(datas::link_tag_data());

    view! {
        <footer class="relative pt-8 pb-6">
            <div class="container mx-auto px-4">
                <div class="flex flex-wrap text-left lg:text-left">
                    <div class="w-full lg:w-6/12 px-4">
                        <div class="flex">
                            <img
                                src="/images/logos/pixel8media-logo-edited-black.png"
                                class="h-16"
                            />
                        </div>
                        <h4 class="text-3xl fonat-semibold text-gray-700">
                            "Let's keep in touch!"
                        </h4>
                        <h5 class="text-lg mt-0 mb-2 text-gray-600">
                            "Find us on any of these platforms, we respond 1-2 business days."
                        </h5>
                        <div class="mt-6 lg:mb-0 mb-6 flex items-center">
                            <A
                                href="https://facebook.com/pxl8media"
                                target="_blank"
                                attr:class="bg-white text-blue-400 shadow-lg font-normal h-10 w-10 flex items-center justify-center align-center rounded-full outline-none focus:outline-none mr-2"
                            >
                                <Icon icon=icondata::BiFacebook width="2rem" height="2rem" />
                            </A>
                            <A
                                href="https://facebook.com/pxl8media"
                                target="_blank"
                                attr:class="bg-white text-blue-400 shadow-lg font-normal h-10 w-10 flex items-center justify-center align-center rounded-full outline-none focus:outline-none mr-2"
                            >
                                <Icon icon=icondata::BiMessenger width="2rem" height="2rem" />
                            </A>
                            <A
                                href="https://www.youtube.com/@LunaGlobalMedia"
                                target="_blank"
                                attr:class="bg-white text-blue-400 shadow-lg font-normal h-10 w-10 flex items-center justify-center align-center rounded-full outline-none focus:outline-none mr-2"
                            >
                                <Icon icon=icondata::BiYoutube width="2rem" height="2rem" />
                            </A>
                        </div>
                    </div>

                    <div class="w-full lg:w-6/12 px-4">
                        <div class="flex flex-wrap items-top mb-6">
                            <div class="w-full lg:w-4/12 px-4 ml-auto">
                                <span class="block uppercase text-sm font-semibold mb-2">
                                    "Useful Links"
                                </span>
                                <ul class="list-unstyled">
                                    <For
                                        each=move || links.get()
                                        key=|links| links.title.clone()
                                        let:link
                                    >
                                        <li>
                                            <LinkTagUI
                                                on_close=move |_| ()
                                                class="text-gray-600 hover:text-gray-800 block pb-2 text-sm capitalize"
                                                title=link.title.clone()
                                                link=link.link.clone()
                                            />

                                        </li>
                                    </For>
                                </ul>
                            </div>
                            <div class="w-full lg:w-4/12 px-4">
                                <span class="block uppercase text-sm mb-2 font-semibold">
                                    "Other Resources"
                                </span>
                                <ul class="list-unstyled">
                                    <li>
                                        <a
                                            class="text-gray-600 hover:text-gray-800 block pb-2 text-sm"
                                            href="/terms-of-service"
                                        >
                                            "Terms of Service"
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="text-gray-600 hover:text-gray-800 block pb-2 text-sm"
                                            href="/privacy-policy"
                                        >
                                            "Privacy Policy"
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="text-gray-600 hover:text-gray-800 block pb-2 text-sm"
                                            href="/refund-policy"
                                        >
                                            "Refund Policy"
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>

                <hr class="my-6 border-blue-300" />

                <div class="flex flex-col lg:flex-row gap-2 items-center justify-center text-center">
                    <span class="text-gray-500">"Copyright © 2025"</span>
                    <span class="text-gray-500">"Created by: "</span>
                    <A
                        href="https://castlebyte.pixl8media.com"
                        target="_blank"
                        attr:class="hover:underline text-blue-500"
                    >
                        "CastleByte Tech Solutions"
                    </A>
                </div>
            </div>
        </footer>
    }.into_any()
}
