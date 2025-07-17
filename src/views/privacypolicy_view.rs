use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn PrivacyPolicyView() -> impl IntoView {
    view! {
        <Title text="Privacy and Policy" />

        <div class="container mx-auto px-4 py-8 max-w-5xl mt-32 mb-12">
            <h1 class="text-4xl font-bold text-center mb-6">
                "Privacy Policy for Pixl8Multimedia"
            </h1>
            <p class="text-sm text-gray-500 text-center">"Effective Date: Jan 2025"</p>

            <p class="mt-4">
                "Pixl8Multimedia (\"we,\" \"our,\" or \"us\") values your privacy and is committed to protecting your personal information. This Privacy Policy explains how we collect, use, and safeguard your information when you visit our website, pixel8media.com (the \"Site\")."
            </p>

            <h2 class="text-xl font-semibold mt-6">"1. Information We Collect"</h2>
            <ul class="list-disc pl-5">
                <li>
                    "Personal Information: Name, email address, phone number, and other details you voluntarily provide when contacting us or signing up for our services."
                </li>
                <li>
                    "Usage Data: Information about your interaction with the Site, such as IP address, browser type, pages viewed, and the time spent on the Site."
                </li>
                <li>
                    "Cookies and Tracking Technologies: We use cookies to enhance your browsing experience and gather analytical data."
                </li>
            </ul>

            <h2 class="text-xl font-semibold mt-6">"2. How We Use Your Information"</h2>
            <ul class="list-disc pl-5">
                <li>"Provide and improve our services."</li>
                <li>"Respond to your inquiries or support requests."</li>
                <li>"Send newsletters, updates, or promotional content, with your consent."</li>
                <li>"Monitor and analyze usage trends to improve the Site."</li>
            </ul>

            <h2 class="text-xl font-semibold mt-6">"3. Sharing Your Information"</h2>
            <ul class="list-disc pl-5">
                <li>
                    "Service Providers: Third-party vendors who assist us in operating the Site and providing services."
                </li>
                <li>"Legal Authorities: When required by law or to protect our legal rights."</li>
            </ul>

            <h2 class="text-xl font-semibold mt-6">"4. Your Privacy Choices"</h2>
            <ul class="list-disc pl-5">
                <li>
                    "Cookies: You can manage your cookie preferences through your browser settings."
                </li>
                <li>
                    "Marketing Communications: Opt-out of promotional emails by following the unsubscribe link in the email."
                </li>
                <li>
                    "Access and Correction: Contact us to access, update, or delete your personal information."
                </li>
            </ul>

            <h2 class="text-xl font-semibold mt-6">"5. Data Security"</h2>
            <p>
                "We implement appropriate security measures to protect your data. However, no method of transmission over the internet or electronic storage is 100% secure."
            </p>

            <h2 class="text-xl font-semibold mt-6">"6. Third-Party Links"</h2>
            <p>
                "Our Site may contain links to third-party websites. We are not responsible for their privacy practices."
            </p>

            <h2 class="text-xl font-semibold mt-6">"7. Children’s Privacy"</h2>
            <p>
                "Our Site is not intended for individuals under 13. We do not knowingly collect personal information from children."
            </p>

            <h2 class="text-xl font-semibold mt-6">"8. Updates to This Policy"</h2>
            <p>
                "We may update this Privacy Policy from time to time. Changes will be posted on this page with an updated effective date."
            </p>

            <h2 class="text-xl font-semibold mt-6">"9. Contact Us"</h2>
            <p>"If you have any questions about this Privacy Policy, please contact us at:"</p>
            <ul class="list-disc pl-5">
                <li>"Pixl8Multimedia"</li>
                <li>
                    "Email: "
                    <A
                        href="mailto:info@pixl8media.com"
                        attr:class="text-blue-600 hover:underline duration-200 ease-in transition"
                    >
                        "info@pixl8media.com"
                    </A>
                </li>
                <li>
                    "Phone: "
                    <A
                        href="tel:+132123467030"
                        attr:class="text-blue-600 hover:underline duration-200 ease-in transition"
                    >
                        "(+1) 3212 346 7030"
                    </A>
                </li>
            </ul>
        </div>
    }
}
