use leptos::prelude::*;

use crate::utils::datas;

#[component]
pub fn ServiceDetailSection() -> impl IntoView {
    let details = RwSignal::new(datas::service_detail_data());

    view! {
        <div class="flex flex-col lg:flex-row gap-12 lg:absolute lg:-top-20 items-center justify-center h-full w-full my-5 lg:my-0 px-6 sm:px-10 md:px-14 lg:px-20">
            <For each=move || details.get() key=|details| details.img.clone() let:child>
                <ServiceDetialCard
                    img=child.img
                    title=child.title
                    content=child.content
                    background=child.background
                />
            </For>
        </div>
    }
}

#[component]
fn ServiceDetialCard(
    #[prop(into)] img: Signal<String>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] content: Signal<String>,
    #[prop(into)] background: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="relative flex flex-col p-8 md:h-80 bg-stone-50 w-full lg:w-96 shadow-lg rounded-md group">
            <div
                class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-all duration-300 ease-in-out bg-cover bg-center z-0 rounded-md"
                style=move || {
                    format!(
                        r##"
                            background: url('{}') no-repeat center/cover,
                                linear-gradient(to top, rgba(20, 20, 20, 0.9) 0%, rgba(0, 0, 0, 0.9) 100%);
                            background-blend-mode: overlay;
                        "##,
                        background.get(),
                    )
                }
                role="img"
            ></div>

            <div class="z-10 flex items-center justify-center lg:absolute p-3 md:rounded-full bg-stone-50 lg:-top-16 lg:left-32 lg:overflow-hidden">
                <div class="h-28 w-28 rounded-full bg-neutral-700 flex items-center justify-center">
                    <img
                        src=format!("/images/{}", img.get())
                        class="md:h-12 w-12"
                        alt=move || title.get().clone()
                    />
                </div>
            </div>
            <div class="mt-5 lg:mt-10 flex flex-col gap-5 lg:gap-10 items-center justify-center text-center z-10 group-hover:text-stone-50">
                <span class="text-2xl font-bold capitalize">{move || title.get()}</span>
                <p class="text-md font-normal text-neutral-500 group-hover:text-neutral-300">
                    {move || content.get()}
                </p>
            </div>
        </div>
    }
}
