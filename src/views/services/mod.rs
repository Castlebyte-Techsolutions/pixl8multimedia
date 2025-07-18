use leptos::prelude::*;
use leptos_router::components::A;
use phosphor_leptos::{Icon, CHECK_SQUARE};

mod upload;

pub use upload::UploadMaterialsView;

#[component]
pub fn ServiceSubmissionView() -> impl IntoView {
    view! {
        <div class="container py-6 px-4 mx-auto mt-24 mb-12 max-w-7xl">
            <h1 class="mb-4 text-3xl font-bold text-center text-blue-800">
                "Submission Guidelines for Screenplay Adaptations"
            </h1>

            <p class="mb-6 text-lg text-gray-800">
                "At Pixl8Media, we are committed to bringing great stories to life. To ensure a smooth and efficient submission process, please follow the guidelines below when submitting materials for screenplay adaptation."
            </p>

            <h2 class="mb-4 text-2xl font-semibold text-gray-800">
                "1. Required Materials"
            </h2>
            <ul class="pl-6 space-y-2 list-disc">
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 items-center font-medium">
                        <Icon
                            icon=CHECK_SQUARE
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Original Source Material – "
                    </span>
                    <span class="text-gray-700">
                        "The book, article, short story, or other material you want to adapt (PDF, Word, or ePub format)."
                    </span>
                </li>
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 items-center font-medium">
                        <Icon
                            icon=CHECK_SQUARE
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Synopsis – "
                    </span>
                    <span class="text-gray-700">
                        "A one-page summary outlining key plot points, characters, and themes."
                    </span>
                </li>
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 items-center font-medium">
                        <Icon
                            icon=CHECK_SQUARE
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Treatment (if available) – "
                    </span>
                    <span class="text-gray-700">
                        "A detailed breakdown of the story, including major scenes and character arcs."
                    </span>
                </li>
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 items-center font-medium">
                        <Icon
                            icon=CHECK_SQUARE
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Logline – "
                    </span>
                    <span class="text-gray-700">
                        "A one- or two-sentence summary capturing the essence of the story."
                    </span>
                </li>
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 items-center font-medium">
                        <Icon
                            icon=CHECK_SQUARE

                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Rights & Permissions – "
                    </span>
                    <span class="text-gray-700">
                        "Proof of ownership or adaptation rights (if the material is not in the public domain)."
                    </span>
                </li>
            </ul>

            <h2 class="mt-6 mb-4 text-2xl font-semibold text-gray-800">
                "2. Formatting Requirements"
            </h2>
            <ul class="pl-6 space-y-2 list-disc">
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 items-center font-medium">
                        <Icon
                            icon=CHECK_SQUARE

                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Screenplays should be in industry-standard format "
                    </span>
                    <span class="text-gray-700">
                        "(Final Draft, Celtx, or PDF)."
                    </span>
                </li>
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 items-center font-medium">
                        <Icon
                            icon=CHECK_SQUARE

                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Text documents must be in a clear, readable font "
                    </span>
                    <span class="text-gray-700">"(12pt, double-spaced)."</span>
                </li>
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 items-center font-medium">
                        <Icon
                            icon=CHECK_SQUARE

                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Visual references (optional) "
                    </span>
                    <span class="text-gray-700">
                        "can be included to enhance the adaptation vision."
                    </span>
                </li>
            </ul>

            <h2 class="mt-6 mb-4 text-2xl font-semibold text-gray-800">
                "3. How to Submit"
            </h2>
            <ul class="pl-6 space-y-2 list-disc">
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 justify-start font-medium">
                        <Icon
                            icon=CHECK_SQUARE
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Email Submission: "
                    </span>
                    <span class="text-gray-700">
                        "Send all required materials to [Your Submission Email]."
                    </span>
                </li>
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 justify-start font-medium">
                        <Icon
                            icon=CHECK_SQUARE
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "File Naming: "
                    </span>
                    <span class="text-gray-700">
                        "Use the format: YourName_ProjectTitle_Submission.pdf."
                    </span>
                </li>
                <li class="flex flex-col gap-2 md:flex-row md:items-start">
                    <span class="flex gap-2 justify-start font-medium">
                        <Icon
                            icon=CHECK_SQUARE
                            attr:class="h-4 w-4 text-green-500"
                        />
                        "Response Time: "
                    </span>
                    <span class="text-gray-700">
                        "We review all submissions within [Timeframe] and will contact you if we are interested in moving forward."
                    </span>
                </li>
            </ul>

            <h2 class="mt-6 mb-4 text-2xl font-semibold text-gray-800">
                "4. Additional Notes"
            </h2>
            <ul class="pl-6 space-y-2 list-disc">
                <li class="flex gap-2 justify-start font-medium">
                    <Icon
                        icon=CHECK_SQUARE
                        attr:class="h-6 w-6 text-green-500"
                    />
                    <span class="text-gray-700">
                        "Ensure all materials are complete and properly formatted before submission."
                    </span>
                </li>
                <li class="flex gap-2 justify-start font-medium">
                    <Icon
                        icon=CHECK_SQUARE
                        attr:class="h-6 w-6 text-green-500"
                    />
                    <span class="text-gray-700">
                        "If your project aligns with our vision, we will schedule a follow-up discussion."
                    </span>
                </li>
                <li class="flex gap-2 justify-start font-medium">
                    <Icon
                        icon=CHECK_SQUARE
                        attr:class="h-6 w-6 text-green-500"
                    />
                    <span class="text-gray-700">
                        "Due to high demand, we may not be able to respond to every submission."
                    </span>
                </li>
            </ul>

            <div class="mt-8 text-center">
                <h3 class="mb-4 text-xl font-semibold text-blue-700">
                    "Ready to submit your screenplay adaptation?"
                </h3>
                <p class="mb-6 text-lg text-gray-800">
                    "Follow the steps above and let’s bring your story to the big screen!"
                </p>

                <A
                    href="upload-materials"
                    attr:class="border border-stone-800 relative overflow-hidden inline-flex items-center bg-transparent duration-200 ease transition hover:rounded-lg px-4 py-2.5 cursor-pointer group"
                >
                    <span class="absolute inset-0 z-0 w-0 h-full transition-all duration-300 ease-in-out group-hover:w-full bg-stone-800"></span>
                    <span class="z-10 transition-all duration-300 ease-in-out group-hover:text-stone-50">
                        "Email us and Submit Now!"
                    </span>
                </A>
            </div>
        </div>
    }
}
