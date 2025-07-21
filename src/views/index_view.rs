use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use phosphor_leptos::{Icon, ARROW_RIGHT};

use crate::{
    components::ui::videos::VideoEmbed,
    sections::{HeroSection, LearnMoreSection, ServiceDetailSection, WorkWithUsSection},
    utils::datas,
};

#[component]
pub fn IndexView() -> impl IntoView {
    view! {
        <Title text="Pixl8Multimedia" />

        <HeroSection />
        <div class="relative lg:h-[380px]">
            <ServiceDetailSection />
        </div>
        <BookTrailerCollectionComponent />
        <LearnMoreSection />
        <WorkWithUsSection />
    }
}

#[component]
fn WorkProcessComponent() -> impl IntoView {
    view! {
        <section class="flex justify-center items-center min-h-screen">
            <div class="flex relative flex-col justify-center items-center max-w-80">
                <div class="flex overflow-hidden relative flex-col gap-5 w-48 h-48 bg-blue-500 rounded-full">
                    <img
                        src="https://picsum.photos/1200/1200"
                        alt=""
                        class="w-full h-full"
                    />
                </div>
                <div class="absolute top-8 right-8 bg-blue-500 rounded-full border-2 border-white h-18 w-18">
                    <div class="flex justify-center items-center h-full text-3xl font-bold text-stone-50">
                        "01"
                    </div>
                </div>
                <h1 class="mt-5 text-2xl font-bold">"Lorem, ipsum dolor."</h1>
                <span class="text-center">
                    "Lorem ipsum dolor sit amet consectetur adipisicing elit. Necessitatibus, voluptatibus."
                </span>
            </div>
        </section>
    }
}

#[component]
fn BookTrailerCollectionComponent() -> impl IntoView {
    let vid_data = RwSignal::new(datas::video_list_data());
    let open_item = RwSignal::new(1);

    let trailer_vid = move || {
        vid_data
            .get()
            .into_iter()
            .take(5)
            .enumerate()
            .map(|(i, video)| {
                let index = i + 1;
                let text_indx = format!("Trailer Collection {index}");
                view! {
					<div class="flex flex-col gap-5 w-full h-full rounded-lg md:flex-row group">
						<div class=move || {
							if open_item.get() == index {
								"flex flex-col overflow-hidden rounded-lg w-full md:w-96 relative transition duration-300 delay-300 ease-in-out"
							} else {
								"hidden transition duration-300 ease-in-out delay-300"
							}
						}>
							<VideoEmbed embed=video.embed indx=index />
							<div class="absolute bottom-0 py-3 w-full text-center text-white bg-gray-600">
								<span class="text-xl md:text-lg">
									{format!("Trailer Collection {index}")}
								</span>
							</div>
						</div>
						<button
							class=move || {
								if open_item.get() != index {
									"flex md:h-full py-2.5 text-xl w-full md:text-lg md:w-16 cursor-pointer items-center justify-center rounded-lg bg-gray-600 text-white transition duration-300 ease-in-out delay-300"
								} else {
									"hidden transition duration-300 ease-in-out delay-300"
								}
							}
							on:click=move |_| open_item.set(index)
						>
							<span class="whitespace-nowrap md:rotate-90">
								{text_indx}
							</span>
						</button>
					</div>
				}.into_any()
            })
            .collect_view()
    };

    view! {
        <section
            id="trailer"
            class="py-12 px-6 space-y-12 bg-gray-200 md:py-24"
        >
            <div class="container grid gap-12 mx-auto md:grid-cols-2">
                <div class="flex flex-col gap-5 text-center">
                    <h1 class="text-5xl font-bold uppercase">
                        "Our Sample Book Trailer Collection"
                    </h1>
                    <span class="text-lg text-gray-500 text-wrap">
                        "Discover The Captivating World Of Our Book Trailer Collection, Where Imagination Comes To Life On The Screen. Dive Into The Pages Of Your Story As We Skillfully Blend Visuals, Music, And Narration Into A Mesmerizing Audiovisual Experience. Our Book Trailer Service Is Designed To Not Just Showcase Your Literary Masterpiece, But To Transport Viewers Into The Essence Of Your Narrative, Enticing Them To Embark On A Journey Through Your Words. Whether You're An Author Seeking To Amplify Your Book's Allure Or A Publisher Aiming To Ignite Curiosity, Our Meticulously Crafted Trailers Promise To Ignite Intrigue And Kindle The Desire To Explore The Realms Of Your Literary Creation. Experience The Magic Of Storytelling In A New Dimension – Come And Immerse Yourself In The Artistry Of Our Book Trailer Collection."
                    </span>

                    <div class="flex justify-center w-full">
                        <A
                            href="/screenplay/trailer"
                            attr:class="rounded-full flex items-center justify-center bg-indigoblue-300 px-6 py-3 text-lg text-white transition text-center hover:bg-indigoblue-600 group"
                        >
                            "View more"
                            <Icon
                                icon=ARROW_RIGHT
                                attr:class="h-4 w-8 ml-2 group-hover:translate-x-1 duration-300 ease-in-out transition"
                            />
                        </A>
                    </div>
                </div>
                <div class="flex relative flex-col gap-5 items-center w-full md:flex-row">
                    {trailer_vid}
                </div>
            </div>
        </section>
    }
}
