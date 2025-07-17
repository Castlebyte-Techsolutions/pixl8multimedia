use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use phosphor_leptos::{Icon, MAGNIFYING_GLASS};

use crate::sections::{
    ProducersPitch, QueryLetter, Representation, Storyboard, Trailer, TreatmentLogline,
};

#[component]
pub fn ScreenplayView() -> impl IntoView {
    let params = use_params_map();
    let adaptation = RwSignal::new(String::from("representation"));
    let adapt = Memo::new(move |_| params.read().get("adaptation").unwrap_or_default());

    Effect::new(move || {
        let temp = adapt.get();
        adaptation.set(temp.clone());
    });

    move || match adaptation.get().as_str() {
        "representation" => view! { <Representation /> }.into_any(),
        "query-letter" => view! { <QueryLetter /> }.into_any(),
        "treatment-logline" => view! { <TreatmentLogline /> }.into_any(),
        "trailer" => view! { <Trailer /> }.into_any(),
        "storyboard" => view! { <Storyboard /> }.into_any(),
        "producer-pitch" => view! { <ProducersPitch /> }.into_any(),
        _ => view! { <Representation /> }.into_any(),
    }
}

#[component]
fn NoAdaptationFound() -> impl IntoView {
    view! {
        <div class="h-full flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold tracking-wider font-playfair capitalize">
                "adaptation not found"
            </h1>
            <Icon icon=MAGNIFYING_GLASS attr:class="h-10 w-10 text-gray-400" />
        </div>
    }
}
