use leptos::prelude::*;

use crate::components::ui::sample_card::SampleCard;

#[component]
pub fn OurWorkProcess() -> impl IntoView {
    view! {
        <section class="flex flex-col gap-5 px-5 container mx-auto">
            <h1 class="text-7xl text-center font-black tracking-wide text-blue-400">
                "Our Proven Work Process"
            </h1>
            <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-10 mt-5">
                <SampleCard
                    title="meeting"
                    content="We start with a discovery session to understand your brand, goals, and audience laying the groundwork for a tailored marketing approach."
                    img="/images/imgs/meeting.png"
                    count=1
                />
                <SampleCard
                    title="strategy"
                    content="We develop a custom marketing plan using data, insights, and proven tactics to ensure your brand stands out and drives results."
                    img="/images/imgs/strategy.png"
                    count=2
                />
                <SampleCard
                    title="implementation"
                    content="Our team brings the strategy to life through campaigns, content, and design executed across the right platforms for maximum impact."
                    img="/images/imgs/implementation.png"
                    count=3
                />
                <SampleCard
                    title="final result"
                    content="We measure results, optimize performance, and deliver transparent reports to ensure your marketing investment keeps working for your growth."
                    img="/images/imgs/final-result.png"
                    count=4
                />
            </div>
        </section>
    }
}
