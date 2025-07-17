use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn TermsOfServiceView() -> impl IntoView {
    view! {
        <Title text="Terms of Service" />

        <div class="mt-32 max-w-5xl mx-auto p-6 bg-white shadow-lg rounded-lg mb-12">
            <h1 class="text-2xl font-bold mb-4">"Terms of Service for Pixl8Media"</h1>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"1. Introduction"</h2>
                <p>
                    "Welcome to Pixl8Media! These Terms of Service (\"Terms\") govern the relationship between Pixel8Media (\"Publisher\") and the advertiser, client, or any agent acting on their behalf (\"Advertiser\"). By placing an order with Pixel8Media, the Advertiser agrees to be bound by these Terms."
                </p>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"2. Orders"</h2>
                <ul class="list-disc pl-6">
                    <li>
                        "All advertising orders (\"Order\") must be made in writing and submitted in the format stipulated by Pixl8Media."
                    </li>
                    <li>
                        "Pixl8Media reserves the right to reject any Order at its sole discretion. If an Order is rejected, the Advertiser will be notified as soon as possible."
                    </li>
                    <li>
                        "Advertisers must submit advertisement materials in the correct format. Failure to do so may result in additional production costs or cancellation of the Order."
                    </li>
                    <li>
                        "All materials must be submitted by the agreed deadline. Late submissions may lead to cancellation or publication without guaranteed accuracy."
                    </li>
                    <li>
                        "Cancellations or modifications must be made in writing at least one week before the deadline; otherwise, the full charge applies."
                    </li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">
                    "3. Advertisement Approval and Placement"
                </h2>
                <ul class="list-disc pl-6">
                    <li>"All advertisements are subject to Pixl8Media's approval."</li>
                    <li>
                        "Placement, positioning, and scheduling are determined at Pixl8Media’s discretion unless otherwise agreed in writing."
                    </li>
                    <li>
                        "The Publisher reserves the right to label advertisements as \"Sponsored\" or \"Advertisement\" when necessary."
                    </li>
                    <li>
                        "Advertisements must comply with all applicable laws, advertising standards, and intellectual property rights."
                    </li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"4. Advertiser’s Responsibilities"</h2>
                <ul class="list-disc pl-6">
                    <li>
                        "The Advertiser warrants that all submitted content is legal, truthful, and non-deceptive."
                    </li>
                    <li>"Does not infringe any third-party intellectual property rights."</li>
                    <li>"Complies with all advertising laws and regulations."</li>
                    <li>"Does not contain offensive, defamatory, or misleading content."</li>
                    <li>
                        "The Advertiser retains copyright over submitted artwork but grants Pixl8Media a license to use it for publication and promotional purposes."
                    </li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"5. Payment Terms"</h2>
                <ul class="list-disc pl-6">
                    <li>
                        "Payment terms are as per Pixl8Media’s rate card unless otherwise agreed in writing."
                    </li>
                    <li>
                        "Payment is due upon order placement unless credit terms have been approved."
                    </li>
                    <li>
                        "Approved credit payments are due within 30 days from the invoice date."
                    </li>
                    <li>
                        "Late payments are subject to a 5% monthly interest above the standard rate."
                    </li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"6. Indemnities and Liability"</h2>
                <ul class="list-disc pl-6">
                    <li>
                        "The Advertiser indemnifies Pixl8Media against all claims, losses, or damages arising from the advertisement’s content or breach of these Terms."
                    </li>
                    <li>
                        "Pixl8Media is not liable for any damages arising from rejected advertisements, errors due to third-party actions, or inaccurate Advertiser instructions."
                    </li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"7. Changes and Termination"</h2>
                <ul class="list-disc pl-6">
                    <li>
                        "Pixl8Media reserves the right to modify these Terms at any time. Advertisers will be notified of material changes."
                    </li>
                    <li>
                        "Either party may terminate the agreement if the other breaches its obligations."
                    </li>
                    <li>"Upon termination, any outstanding payments remain due."</li>
                </ul>
            </section>

            <section class="mb-6">
                <h2 class="text-xl font-semibold mb-2">"8. Governing Law"</h2>
                <p>
                    "These Terms are governed by and construed in accordance with the laws of [Jurisdiction], and any disputes shall be subject to the exclusive jurisdiction of the courts of [Jurisdiction]."
                </p>
            </section>

            <section class="text-center mt-6">
                <p>
                    "For inquiries, contact Pixl8Media at "
                    <A
                        href="mailto:info@pixl8media.com"
                        attr:class="text-blue-600 hover:underline duration-200 ease-in transition"
                    >
                        "info@pixl8media.com"
                    </A>
                </p>
                <p class="font-semibold">"Thank you for choosing Pixl8Media!"</p>
            </section>
        </div>
    }
}
