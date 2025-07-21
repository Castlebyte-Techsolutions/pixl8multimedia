use leptoaster::{provide_toaster, Toaster};
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    ParamSegment, StaticSegment,
};

use crate::{layouts::*, views::*};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="description"
                    content="Get noticed in Hollywood with Pixel8Media's professional screenplay query letter service. Craft compelling pitch letters that open doors to producers and agents."
                />
                <meta
                    name="keywords"
                    content="query letter for screenplay, professional screenplay query letter, screenplay pitch letter, screenwriting services, Pixl8Multimedia, screenwriter query help, Hollywood query letters"
                />
                <meta name="author" content="Pixel8Multimedia" />
                <meta name="robots" content="index, follow" />
                <link rel="canonical" href="https://www.pixel8media.com" />

                <meta
                    property="og:title"
                    content="Professional Screenplay Query Letters | Pixel8Media"
                />
                <meta
                    property="og:description"
                    content="Stand out with a compelling screenplay query letter written by industry experts. Get your script read by Hollywood professionals."
                />
                <meta
                    property="og:image"
                    content="https://www.pixel8media.com/images/logos/pixel8media-logo-edited.png"
                />
                <meta property="og:url" content="https://www.pixel8media.com" />
                <meta property="og:type" content="website" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />

                <Script attr:r#type="application/ld+json">
                    r#"{
					"@context": "https://schema.org",
					"@type": "ProfessionalService",
					"name": "Pixel8Media",
					"url": "https://www.pixel8media.com",
					"logo": "https://www.pixel8media.com/public/logo.png",
					"description": "Pixel8Media helps screenwriters craft industry-standard query letters to submit their screenplay ideas to producers, studios, and agents.",
					"address": {
					"@type": "PostalAddress",
					"addressLocality": "Los Angeles",
					"addressRegion": "CA",
					"postalCode": "90001",
					"addressCountry": "US"
					},
					"contactPoint": {
					"@type": "ContactPoint",
					"telephone": "+1-800-555-QUERY",
					"contactType": "Customer Support",
					"availableLanguage": ["English"]
					},
					"sameAs": [
					"https://www.facebook.com/pixel8media",
					"https://www.instagram.com/pixel8media",
					"https://www.linkedin.com/company/pixel8media"
					]
					}"#
                </Script>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_toaster();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pixl8multimedia.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <Toaster stacked=true />

            <Routes fallback=|| view! { <NotFoundView /> }.into_any()>
                <ParentRoute path=StaticSegment("") view=MainLayout>
                    <Route path=StaticSegment("") view=IndexView />
                    <Route path=StaticSegment("about-us") view=AboutUsView />

                    <Route
                        path=StaticSegment("contact-us")
                        view=ContactUsView
                    />
                    <Route
                        path=StaticSegment("terms-of-service")
                        view=TermsOfServiceView
                    />
                    <Route
                        path=StaticSegment("privacy-policy")
                        view=PrivacyPolicyView
                    />
                    <Route
                        path=StaticSegment("refund-policy")
                        view=RefundPolicyView
                    />

                    <ParentRoute
                        path=StaticSegment("screenplay")
                        view=ScreenplayLayout
                    >
                        <Route path=StaticSegment("") view=ScreenplayView />
                        <Route
                            path=(StaticSegment(""), ParamSegment("adaptation"))
                            view=ScreenplayView
                        />
                    </ParentRoute>

                    <ParentRoute path=StaticSegment("service") view=NoLayout>
                        <Route path=StaticSegment("") view=ServiceSubmissionView />
                        <Route
                            path=StaticSegment("upload-materials")
                            view=UploadMaterialsView
                        />
                    </ParentRoute>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn NotFoundView() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center min-h-screen bg-stone-50 dark:bg-stone-800 dark:text-stone-50">
            <div class="flex gap-5 items-end border-b-4 border-gray-100">
                <h1 class="font-mono text-6xl font-black md:text-8xl lg:text-9xl">
                    "404"
                </h1>
                <Icon
                    icon=icondata::MdiMagnify
                    attr:class="h-32 sm:h-44 md:h-48 w-32 sm:w-44 md:w-48 lg:h-52 lg:w-52"
                />
            </div>
            <span class="my-8 text-xl font-medium">"Page not found"</span>

            <div class="flex items-center">
                <A
                    href="/"
                    attr:class="relative border border-stone-800 px-4 py-2 dark:border-stone-50 inline-flex group"
                >
                    <span class="absolute inset-0 w-full h-0 transition-all duration-300 ease-linear group-hover:h-full bg-stone-800 dark:bg-stone-50"></span>
                    <span class="z-10 group-hover:dark:text-stone-800 group-hover:text-stone-50">
                        "Go back home"
                    </span>
                </A>
            </div>
        </section>
    }
}
