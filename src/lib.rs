pub mod app;
pub mod components;
pub mod error;
pub mod layouts;
pub mod sections;
pub mod templates;
pub mod utils;
pub mod views;

#[cfg(feature = "ssr")]
pub mod app_state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
