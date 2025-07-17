use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::sections::OurWorkProcess;

#[component]
pub fn AboutUsView() -> impl IntoView {
    view! {
        <Title text="About Us" />

        <section
            class="flex min-h-[450px] flex-col items-center justify-center bg-cover bg-center bg-no-repeat"
            style="background-image: linear-gradient(to top, rgba(20, 20, 20, 0.9) 0%, rgba(0, 0, 0, 0.9) 100%), url('/images/imgs/about-us.png');"
        >
            <h1 class="text-center text-7xl font-black tracking-wider text-stone-50 uppercase font-playfair">
                about us
            </h1>
            <h3 class="max-w-4xl text-center text-lg text-stone-300">
                "Why did I start pixl8multimedia with my partner? Because storytelling and visual creativity are at the heart of everything we do. I never imagined myself diving into multimedia production, but the journey became a passion—one fueled by the need to bring ideas to life in the most compelling way possible."
            </h3>
        </section>

        // <span class="text-gray-500 text-md">
        // "Our team is dedicated to providing high-quality multimedia and marketing services, ensuring your brand stands out in a crowded digital space. We walk with you every step of the way, offering honest guidance, creative direction, and a commitment to excellence. No empty promises—just real results."
        // </span>

        <section class="py-12 grid md:grid-cols-2 gap-10 container mx-auto px-5">
            <div class="flex flex-col gap-3">
                <h1 class="text-7xl font-black tracking-wider text-stone-900">
                    We turn your ideas into reality
                </h1>
                <h3 class="text-lg font-medium tracking-wide text-stone-500">
                    "Through years of experience, we’ve seen the challenges creatives face in getting their work noticed. That’s why we built pixl8multimedia—to bridge the gap between vision and execution, offering innovative solutions in photography, videography, branding, and digital content creation."
                </h3>

                <A
                    href="/contact-us"
                    attr:class="mt-10 items-center justify-center relative overflow-hidden inline-flex px-4 py-2.5 border border-stone-900 group rounded-md"
                >
                    <span class="absolute bottom-0 left-0 h-0 w-full transition-all duration-300 ease-in-out group-hover:h-full bg-stone-900"></span>
                    <span class="z-10 text-xl font-medium tracking-wide group-hover:text-stone-50 duration-300 transition-colors ease-in-out capitalize">
                        "contact us"
                    </span>
                </A>
            </div>

            <div class="grid grid-cols-1 xl:grid-cols-2 gap-5">
                <div class="flex bg-cover bg-center h-96 lg:h-full bg-no-repeat bg-[url('/images/imgs/about-us-02.png')] rounded-md"></div>
                <div class="flex bg-cover bg-center h-96 lg:h-full bg-no-repeat bg-[url('/images/imgs/about-us-03.png')] rounded-md"></div>
            </div>
        </section>

        <OurWorkProcess />

        <section class="container mx-auto px-5 gap-5 py-24 grid grid-cols-1 xl:grid-cols-2">
            <div class="bg-cover bg-no-repeat bg-center bg-[url('/images/imgs/about-us-04.png')] h-[450px] rounded-md"></div>
            <div class="flex flex-col gap-4 justify-center px-5">
                <h1 class="text-6xl font-black uppercase text-indigoblue-300 tracking-wide">
                    "be a pexil8media"
                </h1>
                <h3 class="text-xl font-medium text-stone-500">
                    "Pixl8Media functions as a platform which turns stories into genuine visual representations that outstrip their textual manifestation. Authors who work for film production must develop their unique voice which requires worldwide distribution. Build a place within our team to assist development of narratives beyond mainstream standards and create meaningful connections with audience."
                </h3>

                <A
                    href="/contact-us"
                    attr:class="mt-10 items-center justify-center relative overflow-hidden inline-flex px-4 py-2.5 border border-stone-900 group rounded-md"
                >
                    <span class="absolute bottom-0 left-0 h-0 w-full transition-all duration-300 ease-in-out group-hover:h-full bg-stone-900"></span>
                    <span class="z-10 text-xl font-medium tracking-wide group-hover:text-stone-50 duration-300 transition-colors ease-in-out capitalize">
                        "contact us"
                    </span>
                </A>
            </div>
        </section>
    }
}
