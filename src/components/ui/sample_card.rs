use leptos::prelude::*;

#[component]
pub fn SampleCard(
    #[prop(into)] title: Signal<String>,
    #[prop(into)] content: Signal<String>,
    #[prop(into)] img: Signal<String>,
    #[prop(into)] count: Signal<i32>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col  items-center">
            <div class="h-32 w-32 rounded-full relative bg-indigoblue-200 flex items-center justify-center">
                <div class="z-10 h-12 w-12 rounded-full absolute top-2 bg-indigoblue-200 border-2 border-white -right-5 flex items-center justify-center font-black text-xl">
                    {move || format!("0{}", count.get())}
                </div>
                <img
                    src=move || img.get()
                    alt=""
                    class="h-full w-full rounded-full z-10 p-3 object-center"
                />
            </div>
            <h1 class="capitalize text-xl font-bold tracking-wide mt-3">{move || title.get()}</h1>
            <h3 class="max-w-72 text-center text-base text-neutral-600">{move || content.get()}</h3>
        </div>
    }
}
