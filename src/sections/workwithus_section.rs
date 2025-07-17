use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn WorkWithUsSection() -> impl IntoView {
    view! {
        <div class="py-24 bg-gray-200 px-6 sm:px-10 md:px-14 lg:px-20">
            <div class="container mx-auto">
                <div class="flex flex-col gap-5 md:flex-row items-center justify-between">
                    <div class="flex flex-col gap-5">
                        <h1 class="text-2xl font-semibold">
                            "Ready To Start New Project With Pixl8Multimedia?"
                        </h1>
                        <span class="text-sm font-normal">
                            "Are you ready to bring your creative vision to life with Pixl8Multimedia? Let’s collaborate and craft something truly extraordinary together!"
                        </span>
                    </div>
                    <div class="flex items-center justify-end">
                        <A
                            href="contact-us"
                            attr:class="px-5 py-2.5 border border-stone-700 bg-transparent relative inline-flex overflow-hidden group"
                        >
                            <span class="absolute z-0 inset-0 w-0 h-full bg-stone-700 transition-all duration-300 delay-200 ease-in-out group-hover:w-full"></span>
                            <span class="z-10 group-hover:text-stone-50 transition-colors duration-300 delay-200 ease-in-out">
                                "Contact Us"
                            </span>
                        </A>
                    </div>
                </div>
            </div>
        </div>
    }
}
