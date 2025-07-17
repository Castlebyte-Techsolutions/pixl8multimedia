use leptos::prelude::*;
use leptos_router::components::A;
use phosphor_leptos::{Icon, CARET_DOUBLE_DOWN};

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="min-h-screen flex flex-col items-center justify-center gap-10 bg-change-5x px-5">
            <h1 class="text-4xl md:text-5xl lg:text-6xl xl:text-7xl text-left md:text-center font-black tracking-wider text-stone-50">
                "Pixl8Multimedia"
            </h1>
            <span class="text-xl tracking-wide text-left sm:text-center max-w-5xl text-stone-200">
                "At Pixl8Multimedia, we are fueled by a relentless passion for innovation and a steadfast commitment to excellence, constantly pushing boundaries to deliver creative and impactful solutions."
            </span>
            <A
                href="#trailer"
                attr:class="group relative mt-10 inline-flex h-12 w-44 items-center justify-center overflow-hidden rounded-full bg-indigoblue-300 px-5 py-2.5 text-xl font-bold text-gray-50 uppercase"
            >
                <span class="absolute transition-all duration-300 ease-in-out group-hover:-translate-y-3 group-hover:opacity-0">
                    "start now"
                </span>
                <Icon
                    icon=CARET_DOUBLE_DOWN
                    attr:class="absolute h-5 w-5 translate-y-3 opacity-0 transition-all duration-300 ease-in-out group-hover:translate-y-0 group-hover:opacity-100"
                />

            </A>
        </section>
    }
}
