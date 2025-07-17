use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

use crate::{components::ui::videos::VideoEmbed, utils::datas};

#[component]
pub fn Representation() -> impl IntoView {
    view! {
        <div class="container mx-auto p-8 my-12 rounded-lg shadow-lg">
            <h1 class="text-4xl font-bold text-center text-blue-600 mb-6">
                "Representation in Screenplay Adaptation: Bringing Stories to Life"
            </h1>

            <p class="text-center text-gray-700 mb-8 text-lg">
                "At Pixl8Media, we transform ideas into cinematic masterpieces. Whether adapting books, comics, or real-life events, we ensure your story captivates audiences and meets industry standards."
            </p>

            <div class="grid md:grid-cols-2 gap-6">
                <RepresentationFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Authentic Character Development"
                    description="We create multi-dimensional characters with depth, personality, and emotional appeal."
                />
                <RepresentationFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Cinematic Storytelling"
                    description="Seamlessly transitioning narratives for the silver screen with pacing, tension, and drama."
                />
                <RepresentationFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Cultural & Emotional Depth"
                    description="Keeping the heart of your story intact while broadening its audience appeal."
                />
                <RepresentationFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Visual World-Building"
                    description="From atmospheric settings to striking aesthetics, we turn your vision into immersive storytelling."
                />
                <RepresentationFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Innovative Screenwriting"
                    description="Crafting scripts using industry-standard techniques for impactful storytelling."
                />
            </div>

            <div class="mt-10">
                <h2 class="text-2xl font-semibold text-blue-500">
                    "Sample Adaptation – From Novel to Screenplay"
                </h2>
                <div class="bg-white p-6 rounded-lg mt-4 shadow">
                    <h3 class="text-lg font-semibold text-gray-800">"Before: Novel Excerpt"</h3>
                    <p class="text-gray-600 italic">
                        "The rain dripped down the cracked pavement as Detective Carter watched the city lights fade. He had one chance to catch the killer before everything collapsed."
                    </p>
                </div>
                <div class="bg-white p-6 rounded-lg mt-4 shadow fade-in">
                    <h3 class="text-lg font-semibold text-gray-800">
                        "After: Screenplay Adaptation (Scene Format)"
                    </h3>
                    <pre class="text-gray-700 bg-gray-200 p-3 rounded-md overflow-x-auto">
                        "EXT. CITY STREET – NIGHT
                        Rain pours onto the cracked pavement. Detective Carter leans against his car, 
                        eyes locked on the city skyline. The neon signs flicker. He exhales, steeling himself."
                    </pre>
                </div>
            </div>

            <div class="text-center mt-8 flex flex-col gap-2 items-center justify-center">
                <h3 class="text-lg font-semibold text-blue-500">
                    "Ready to Bring Your Vision to Life?"
                </h3>
                <p class="text-gray-700">
                    "Let’s turn your story into a cinematic experience with expert screenplay adaptation."
                </p>

                <A
                    href="/contact-us"
                    attr:class="border border-stone-800 bg-transparent hover:bg-stone-800 hover:text-white duration-200 ease transition hover:rounded-lg px-4 py-2.5 cursor-pointer"
                >
                    "Let’s start planning your scenes—contact us today!"
                </A>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn QueryLetter() -> impl IntoView {
    view! {
        <div class="container mx-auto p-8 my-12 rounded-lg shadow-lg">
            <h1 class="text-4xl font-bold text-center text-blue-600 mb-6">
                "Get Noticed with a Professional Query Letter for Your Screenplay"
            </h1>

            <p class="text-center text-gray-700 mb-8 text-lg">
                "A query letter is your golden ticket into the film industry! It serves as your first impression when reaching out to agents, producers, and studios. At Pixl8Media, we craft industry-standard query letters that demand attention and open doors for your screenplay adaptation."
            </p>

            <div class="grid md:grid-cols-2 gap-6">
                <QueryLetterFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Captivating Storytelling"
                    description="We highlight the most compelling aspects of your screenplay to spark interest."
                />
                <QueryLetterFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Hollywood-Standard Formatting"
                    description="Professionally structured letters that align with industry expectations."
                />
                <QueryLetterFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Tailored to Your Vision"
                    description="Your story, your voice—crafted to engage and inspire decision-makers."
                />
                <QueryLetterFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Results-Driven Call to Action"
                    description="Our persuasive strategies help convert interest into opportunity."
                />
            </div>

            <div class="mt-10">
                <h2 class="text-2xl font-semibold text-blue-500">"Sample Query Letter Excerpt"</h2>
                <div class="bg-gray-200 p-6 rounded-lg mt-4">
                    <h3 class="text-lg font-semibold text-gray-800">
                        "Before: Generic and Unengaging"
                    </h3>
                    <p class="text-gray-600 italic">
                        "Dear Sir/Madam, I have written a screenplay that I believe would interest your company. It is a drama about a struggling artist. Please let me know if you are interested."
                    </p>
                </div>
                <div class="bg-gray-200 p-6 rounded-lg mt-4 fade-in">
                    <h3 class="text-lg font-semibold text-gray-800">
                        "After: Professional and Engaging"
                    </h3>
                    <pre class="text-gray-700 bg-gray-300 p-3 rounded-md overflow-x-auto">
                        "Subject: Compelling Drama 'Unbroken Canvas' – Query for Representation\n\nDear [Agent/Producer’s Name],\n\nI hope this email finds you well. I am reaching out to introduce 'Unbroken Canvas,' a character-driven drama that explores the passion and struggles of an artist striving to find their place in the world. With a tone reminiscent of 'Whiplash' and the raw intensity of 'Black Swan', this screenplay delves into the highs and lows of artistic ambition.\n\nI would love the opportunity to share the full script with you. Please let me know if you'd be interested in reading it.\n\nBest regards,\n[Your Name]"
                    </pre>
                </div>
            </div>

            <div class="text-center mt-8 flex flex-col gap-2 items-center justify-center">
                <h3 class="text-lg font-semibold text-blue-500">"Ready to Get Noticed?"</h3>
                <p class="text-gray-700">
                    "Let’s make your screenplay shine and get the attention it deserves!"
                </p>

                <A
                    href="/contact-us"
                    attr:class="border border-stone-800 bg-transparent hover:bg-stone-800 hover:text-white duration-200 ease transition hover:rounded-lg px-4 py-2.5 cursor-pointer"
                >
                    "Let’s start planning your scenes—contact us today!"
                </A>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn TreatmentLogline() -> impl IntoView {
    view! {
        <div class="container mx-auto p-8 rounded-lg shadow-lg my-12 text-gray-900">
            <h1 class="text-4xl font-bold text-center text-blue-600 mb-6">
                "Craft the Perfect Treatment & Logline for Your Screenplay Adaptation with Pixl8Media"
            </h1>

            <p class="text-center text-gray-700 mb-8 text-lg">
                "At Pixl8Media, we craft industry-standard treatments and loglines that turn your screenplay into an irresistible pitch. Whether you’re presenting to producers, investors, or major studios, we ensure your story stands out in a crowded marketplace."
            </p>

            <div class="grid md:grid-cols-2 gap-6">
                <LoglineFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Engaging Treatments"
                    description="A professionally structured story outline that enhances your screenplay’s appeal."
                />
                <LoglineFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Impactful Loglines"
                    description="A compelling, one-sentence hook that captures the heart of your story."
                />
                <LoglineFeatureBox
                    icon=icondata::BiCheckDoubleRegular
                    title="Tailored Storytelling"
                    description="Every treatment and logline is meticulously crafted to match your vision while meeting industry expectations."
                />
            </div>

            <div class="mt-10">
                <h2 class="text-2xl font-semibold text-blue-500">
                    "Sample Logline & Treatment Excerpt"
                </h2>
                <div class="bg-white p-6 rounded-lg mt-4 shadow">
                    <h3 class="text-lg font-semibold text-gray-800">"Before: Basic & Unclear"</h3>
                    <p class="text-gray-600 italic">
                        "A detective hunts a killer in a futuristic city, but his past haunts him."
                    </p>
                </div>
                <div class="bg-white p-6 rounded-lg mt-4 shadow fade-in">
                    <h3 class="text-lg font-semibold text-gray-800">
                        "After: Refined & Compelling"
                    </h3>
                    <p class="text-gray-700">
                        "Logline: In a neon-drenched dystopia, a tormented detective must track down a rogue AI serial killer—only to discover it was programmed using his own memories."
                    </p>
                    <p class="mt-2 text-gray-700">
                        "Treatment Excerpt: Detective Kade Hunter thought he had buried his past, but when a new wave of cybernetic murders grips Neo-Tokyo, the case pulls him into a psychological maze. The AI killer isn't just mimicking human behavior—it’s using his very own thoughts and memories, forcing him to confront the darkest corners of his mind. As he races against time, Kade must choose: stop the AI or unravel the secrets of his past that could destroy him."
                    </p>
                </div>
            </div>

            <div class="text-center mt-8 flex flex-col gap-2 items-center justify-center">
                <h3 class="text-lg font-semibold text-blue-500">
                    "Ready to Elevate Your Screenplay?"
                </h3>
                <p class="text-gray-700">
                    "Let’s transform your treatment and logline into a powerful pitch that grabs attention!"
                </p>
                <A
                    href="/contact-us"
                    attr:class="border border-stone-800 bg-transparent hover:bg-stone-800 hover:text-white duration-200 ease transition hover:rounded-lg px-4 py-2.5 cursor-pointer"
                >
                    "Contact Us Today!"
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn Trailer() -> impl IntoView {
    let videos = RwSignal::new(datas::video_list_data());

    let vid_1 = videos
        .get()
        .into_iter()
        .take(4)
        .map(|y| {
            view! { <VideoEmbed embed=y.embed.clone() indx=y.id /> }
        })
        .collect_view();

    let vid_2 = videos
        .get()
        .into_iter()
        .skip(4)
        .map(|x| view! { <VideoEmbed embed=x.embed.clone() indx=x.id /> })
        .collect_view();

    view! {
        <div class="flex flex-col mx-auto container p-5 gap-12">
            <div class="flex flex-col items-center justify-center">
                <h1 class="uppercase text-3xl text-semibold text-center">
                    "our book trailer collection"
                </h1>
                <p class="text-gray-500 text-center mt-5">
                    "Discover the captivating world of our book trailer collection, where imagination comes to life
                    on the screen. Dive into the pages of your story as we skillfully blend visuals, music, and
                    narration into a mesmerizing audiovisual experience. Our book trailer service is designed to
                    not just showcase your literary masterpiece, but to transport viewers into the essence of your
                    narrative, enticing them to embark on a journey through your words. Whether you’re an author
                    seeking to amplify your book’s allure or a publisher aiming to ignite curiosity, our
                    meticulously crafted trailers promise to ignite intrigue and kindle the desire to explore the
                    realms of your literary creation. Experience the magic of storytelling in a new dimension –
                    come and immerse yourself in the artistry of our book trailer collection."
                </p>
            </div>

            <div class="flex flex-col gap-5 max-w-7xl">
                <div class="flex flex-col md:grid md:grid-cols-2 lg:grid-cols-4 gap-5">{vid_1}</div>
                <div class="flex flex-col md:grid md:grid-cols-2 lg:grid-cols-3 gap-5">{vid_2}</div>
            </div>
            <div class="flex items-center justify-center">
                <A
                    href="/contact-us"
                    attr:class="border border-stone-800 bg-transparent hover:bg-stone-800 hover:text-white duration-200 ease transition hover:rounded-lg px-4 py-2.5 cursor-pointer"
                >
                    "Get Quote"
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn Storyboard() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-6 my-12 max-w-7xl">
            <h1 class="text-3xl font-bold text-center text-blue-800 mb-4">
                "Storyboard Development for Screenplay Adaptations"
            </h1>

            <p class="text-lg text-gray-800 mb-6">
                "A storyboard serves as an essential tool which converts an original screenplay into meaningful graphic guides for film production. The breakdown of every scene into unique frames enables storyboarding to show directors along with producers and filmmakers what actions and character motions will appear during production time."
            </p>

            <h2 class="text-2xl font-semibold text-gray-800 mb-4">
                "How Storyboarding Enhances Your Adaptation"
            </h2>
            <ul class="pl-6 space-y-2">
                <li>
                    <span class="font-medium flex items-center gap-2">
                        <Icon
                            icon=icondata::BiCheckDoubleRegular
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Clear Scene Composition – "
                    </span>
                    "Each panel demonstrates a particular camera angle as it shows both visual elements and character site."
                </li>
                <li>
                    <span class="font-medium flex items-center gap-2">
                        <Icon
                            icon=icondata::BiCheckDoubleRegular
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Efficient Production Planning – "
                    </span>
                    "This tool allows directors and cinematographers to synchronize their vision for visual presentation before the production phase begins."
                </li>
                <li>
                    <span class="font-medium flex items-center gap-2">
                        <Icon
                            icon=icondata::BiCheckDoubleRegular
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Stronger Narrative Flow – "
                    </span>
                    "The game engine offers nearby facilities and navigation tools for seamless movement between scenes which also enables proper timing of essential story developments."
                </li>
            </ul>

            <h2 class="text-2xl font-semibold text-gray-800 mt-6 mb-4">
                "Custom Storyboard Writing Services"
            </h2>
            <p class="text-lg text-gray-800 mb-4">
                "The project does not include visual illustrations but includes written storyboards containing structured descriptions which explain every scene presentation:"
            </p>

            <ul class="pl-6 space-y-2">
                <li>
                    <span class="font-medium flex items-center gap-2">
                        <Icon
                            icon=icondata::BiCheckDoubleRegular
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Genre & Tone – "
                    </span>
                    "Setting the right mood for your adaptation."
                </li>
                <li>
                    <span class="font-medium flex items-center gap-2">
                        <Icon
                            icon=icondata::BiCheckDoubleRegular
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Setting & Locations – "
                    </span>
                    "Every scene needs its own environment description through Setting & Locations."
                </li>
                <li>
                    <span class="font-medium flex items-center gap-2">
                        <Icon
                            icon=icondata::BiCheckDoubleRegular
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Character Actions & Expressions – "
                    </span>
                    "The description of situational directions with specific angles and perspectives and transitional elements appear in this section."
                </li>
                <li>
                    <span class="font-medium flex items-center gap-2">
                        <Icon
                            icon=icondata::BiCheckDoubleRegular
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Camera Directions – "
                    </span>
                    "Camera Directions include describing the angles together with perspectives and transitional movements."
                </li>
            </ul>

            <StoryboardSample />

            <div class="text-center mt-8 flex flex-col gap-2 items-center justify-center">
                <h3 class="text-xl font-semibold text-blue-700 mb-4">
                    "Have a screenplay adaptation in mind?"
                </h3>
                <p class="text-lg text-gray-800 mb-6">
                    "Provide us with scene details, and we’ll craft a structured written storyboard to bring your vision to life!"
                </p>

                <A
                    href="/contact-us"
                    attr:class="border border-stone-800 bg-transparent hover:bg-stone-800 hover:text-white duration-200 ease transition hover:rounded-lg px-4 py-2.5 cursor-pointer"
                >
                    "Let’s start planning your scenes—contact us today!"
                </A>
            </div>
        </div>
    }.into_any()
}

#[component]
fn StoryboardSample() -> impl IntoView {
    view! {
        <div class="p-6 bg-gray-900 text-white rounded-lg shadow-lg my-10">
            <h1 class="text-3xl font-bold text-center text-yellow-500 mb-6">
                "Sample Storyboard – Action Thriller Scene"
            </h1>
            <p class="text-center text-gray-300 mb-8">
                "A high-stakes rooftop chase unfolds in the heart of the city. Rain pours as two figures race across the skyline..."
            </p>

            <div class="space-y-8">
                <StoryboardPanel
                    title="Panel 1 – Establishing Shot"
                    description="Wide-angle shot of the city skyline under a stormy sky. Rain reflects neon lights off the rooftops."
                    camera="Camera: Slow pan across the rooftops, setting a tense atmosphere."
                    action="Character: Viktor, in a dark hoodie, runs across a rooftop edge."
                    image_url="/images/storyboard/panel-1.webp"
                />

                <StoryboardPanel
                    title="Panel 2 – Close-Up"
                    description="Detective Carter, breathing heavily, spots Viktor ahead. His determined eyes narrow."
                    camera="Camera: Tight close-up, slight handheld shake to add urgency."
                    action="Character: Carter grips his gun, preparing to chase."
                    image_url="/images/storyboard/panel-2.webp"
                />

                <StoryboardPanel
                    title="Panel 3 – High-Speed Chase"
                    description="Viktor leaps over a gap between two buildings. Carter follows but lands hard."
                    camera="Camera: Over-the-shoulder tracking shot, shaky cam for intensity."
                    action="Character: Carter grimaces, pushing himself up to keep running."
                    image_url="/images/storyboard/panel-3.webp"
                />

                <StoryboardPanel
                    title="Panel 4 – Confrontation"
                    description="Viktor turns, realizing he’s cornered at the rooftop’s edge. Rain drips down his face."
                    camera="Camera: Low-angle shot, making Carter appear dominant."
                    action="Character: Carter shouts, gun drawn, as Viktor hesitates."
                    image_url="/images/storyboard/panel-4.webp"
                />
            </div>
        </div>
    }.into_any()
}

#[component]
fn StoryboardPanel(
    #[prop(into)] title: Signal<&'static str>,
    #[prop(into)] description: Signal<&'static str>,
    #[prop(into)] camera: Signal<&'static str>,
    #[prop(into)] action: Signal<&'static str>,
    #[prop(into)] image_url: Signal<&'static str>,
) -> impl IntoView {
    view! {
        <div class="bg-gray-800 p-4 rounded-lg shadow-lg border border-gray-700 transform transition-all duration-500 hover:scale-105 fade-in">
            <h2 class="text-xl font-semibold text-yellow-400">{move || title.get()}</h2>
            <img
                src=move || image_url.get()
                alt="Storyboard Scene"
                class="w-full h-96 aspect-5/4 object-cover rounded-md my-3 shadow-md"
            />
            <p class="text-gray-300">{move || description.get()}</p>
            <p class="text-gray-400 mt-2">
                <strong>{move || camera.get()}</strong>
            </p>
            <p class="text-gray-400">
                <strong>{move || action.get()}</strong>
            </p>
        </div>
    }
}

#[component]
pub fn ProducersPitch() -> impl IntoView {
    view! {
        <div class="container mx-auto p-8 my-12 rounded-lg shadow-lg max-w-7xl">
            <h1 class="text-3xl font-bold text-center text-blue-600 mb-6">
                "What is a Producer’s Pitch in a Screenplay Adaptation?"
            </h1>

            <p class="text-gray-700 text-lg mb-6 text-center">
                "A producer’s pitch is a concise, compelling presentation used to sell a screenplay adaptation to producers, studios, or investors. It highlights the story’s potential, marketability, and unique appeal while addressing production elements such as target audience, budget, and genre positioning."
            </p>

            <h2 class="text-2xl font-semibold text-blue-500">
                "Producer’s Pitch Sample – Neon Eclipse"
            </h2>

            <div class="mt-4 p-6 bg-white rounded-lg shadow-md">
                <h3 class="text-xl font-semibold text-gray-800">"Logline:"</h3>
                <p class="text-gray-600 italic">
                    "In a neon-drenched dystopian future, a rogue AI is wiping out high-ranking officials, leaving behind cryptic messages from an ex-detective’s past. As he’s forced back into the shadows, he must uncover the truth before the AI learns to outthink him… forever."
                </p>
            </div>

            <div class="mt-4 p-6 bg-gray-50 rounded-lg shadow-md">
                <h3 class="text-xl font-semibold text-gray-800">"Why This Adaptation Works:"</h3>
                <ul class="list-disc list-inside text-gray-700">
                    <li>
                        <strong>"A High-Concept Thriller"</strong>
                        " – Cyberpunk meets neo-noir with a character-driven mystery."
                    </li>
                    <li>
                        <strong>"Strong Audience Appeal"</strong>
                        " – Fans of <i>Blade Runner</i>, <i>Ghost in the Shell</i>, and <i>Westworld</i> will be drawn to its atmospheric world and mind-bending twists."
                    </li>
                    <li>
                        <strong>"Current Market Trend"</strong>
                        " – AI-driven thrillers are in high demand, making this adaptation both timely and commercially viable."
                    </li>
                </ul>
            </div>

            <div class="mt-4 p-6 bg-white rounded-lg shadow-md">
                <h3 class="text-xl font-semibold text-gray-800">"Visual & Tonal Inspiration:"</h3>
                <p class="text-gray-600">
                    "Think the neon-lit streets of <i>Blade Runner 2049</i> mixed with the psychological tension of <i>Se7en</i> and the high-stakes action of <i>John Wick</i>."
                </p>
            </div>

            <div class="mt-4 p-6 bg-gray-50 rounded-lg shadow-md">
                <h3 class="text-xl font-semibold text-gray-800">"Production Scope:"</h3>
                <ul class="list-disc list-inside text-gray-700">
                    <li>
                        <strong>"Estimated Budget:"</strong>
                        "$40M – $60M (CGI-enhanced environments, practical stunts, and a neo-noir aesthetic)"
                    </li>
                    <li>
                        <strong>"Target Audience:"</strong>
                        "Sci-fi & thriller fans, ages 18–45"
                    </li>
                    <li>
                        <strong>"Awards Potential:"</strong>
                        "Strong script with psychological depth and prestige festival appeal"
                    </li>
                </ul>
            </div>

            <div class="mt-6 p-6 bg-blue-100 rounded-lg shadow-md">
                <h3 class="text-xl font-semibold text-blue-700">"Closing Statement:"</h3>
                <p class="text-gray-700">
                    "Neon Eclipse is not just another sci-fi thriller—it’s a thought-provoking exploration of AI ethics, memory, and identity wrapped in a high-intensity action mystery. With the right creative team, this adaptation has the potential to become a modern cult classic. Let’s make it happen!"
                </p>
            </div>

            <div class="text-center mt-8 flex flex-col gap-2 items-center justify-center">
                <h3 class="text-lg font-semibold text-blue-500">
                    "Let Pixl8Media Craft Your Perfect Producer’s Pitch!"
                </h3>
                <p class="text-gray-600">
                    "A great pitch is the key to funding and production success. We’ll help you craft a powerful, polished, and persuasive producer’s pitch that gets results."
                </p>

                <A
                    href="/contact-us"
                    attr:class="border border-stone-800 bg-transparent hover:bg-stone-800 hover:text-white duration-200 ease transition hover:rounded-lg px-4 py-2.5 cursor-pointer"
                >
                    "Contact Us Today!"
                </A>
            </div>
        </div>
    }
}

#[component]
fn RepresentationFeatureBox(
    #[prop(into)] icon: icondata_core::Icon,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white p-4 rounded-lg shadow-lg border border-gray-300 transform transition-all duration-500 hover:scale-105 fade-in">
            <div class="flex items-center gap-2">
                <Icon icon attr:class="h-5 w-5 text-green-500" />
                <h2 class="text-lg font-semibold text-blue-500">{title}</h2>
            </div>
            <p class="text-gray-600 mt-2">{description}</p>
        </div>
    }
}

#[component]
fn QueryLetterFeatureBox(
    #[prop(into)] icon: icondata_core::Icon,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white p-4 rounded-lg shadow-md border border-gray-300 transition-all duration-500 hover:scale-105 fade-in">
            <div class="flex items-center gap-2">
                <Icon icon attr:class="h-6 w-6 text-green-500" />
                <h2 class="text-lg font-semibold text-blue-600">{title}</h2>
            </div>
            <p class="text-gray-600 mt-1">{description}</p>
        </div>
    }
}

#[component]
fn LoglineFeatureBox(
    #[prop(into)] icon: icondata_core::Icon,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-gray-200 p-4 rounded-lg gap-4 shadow-md border border-gray-300 transform transition-all duration-500 hover:scale-105 fade-in">
            <div class="flex items-center gap-2">
                <Icon icon attr:class="h-6 w-6 text-blue-500" />
                <h2 class="text-lg font-semibold text-gray-900">{title}</h2>
            </div>
            <p class="text-gray-700 mt-1">{description}</p>
        </div>
    }
}
