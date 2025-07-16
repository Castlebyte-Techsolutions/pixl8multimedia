use leptoaster::{provide_toaster, expect_toaster, ToastBuilder, Toaster};
use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="utf-8" />
				<meta
					name="viewport"
					content="width=device-width, initial-scale=1"
				/>
				<AutoReload options=options.clone() />
				<HydrationScripts options />
				<MetaTags />
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
			<Toaster />
			<main class="min-h-screen">
				<Routes fallback=|| "Page not found.".into_view()>
					<Route path=StaticSegment("") view=HomePage />
				</Routes>
			</main>
		</Router>
	}
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    #[server]
    pub async fn check_db_health() -> Result<String, ServerFnError> {
        use crate::app_state::AppState;
        let db_pool = expect_context::<AppState>().db_pool;

        let row: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(&db_pool)
            .await
            .unwrap();

        Ok(format!("Database health check row returned: {}", row.0))
    }
    let count = RwSignal::new(0);
    let check_db = RwSignal::new(String::new());
    let on_click = move |_| *count.write() += 1;
    let toaster = expect_toaster();

    let check_health =  move |_| {
    	let toaster = toaster.clone();
					spawn_local(async move {
						let returned_val = check_db_health().await.unwrap();
					toaster.toast(
						ToastBuilder::new(returned_val.to_string()).with_level(leptoaster::ToastLevel::Info).with_position(leptoaster::ToastPosition::TopRight).with_expiry(Some(4_500))
					);
					});
				};
    view! {
		<div class="flex flex-col gap-5 justify-center items-center mx-auto min-h-screen max-w-52">
			<h1>"Welcome to Leptos!"</h1>
			<button class="p-4 w-full bg-stone-500/20" on:click=on_click>
				"Click Me: "
				{count}
			</button>

			<button class="p-4 w-full bg-stone-500/20" on:click=check_health>
				"Check DB"
			</button>
		</div>
	}
}
