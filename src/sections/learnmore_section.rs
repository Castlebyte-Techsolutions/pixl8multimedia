use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn LearnMoreSection() -> impl IntoView {
    view! {
        <div class="container mx-auto py-12 md:py-24 px-6 sm:px-10 md:px-14 lg:px-20">
            <div class="grid md:grid-cols-2 gap-10 place-items-center">
                <div class="flex h-100 w-full bg-[url('/images/imgs/learn-more.png')] bg-cover bg-center bg-no-repeat"></div>
                <div class="flex flex-col gap-5">
                    <h1 class="uppercase text-3xl font-bold text-sky-500">
                        "be a pixl8multimedia"
                    </h1>
                    <span class="text-md text-gray-500">
                        "Pixl8Multimedia functions as a platform which turns stories into genuine visual representations that outstrip their textual manifestation. Authors who work for film production must develop their unique voice which requires worldwide distribution. Build a place within our team to assist development of narratives beyond mainstream standards and create meaningful connections with audience."
                    </span>
                    <div class="flex items-center">
                        <A
                            href="about-us"
                            attr:class="relative inline-flex items-center overflow-hidden px-5 py-2.5 border border-skyblue-300 uppercase group"
                        >
                            <span class="inset-0 absolute bg-skyblue-300 z-0 w-0 h-full delay-100 transition-all duration-300 ease-in-out group-hover:w-full"></span>
                            <span class="z-10 delay-200 group-hover:text-stone-50 tracking-wide">
                                "learn more"
                            </span>
                        </A>
                    </div>
                </div>
            </div>
        </div>
    }
}
