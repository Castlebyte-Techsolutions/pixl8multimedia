// mod contact_section;
mod footer_section;
mod header_section;
mod hero_section;
mod learnmore_section;
mod screenplay_section;
mod servicedetail_section;
mod workprocess_section;
mod workwithus_section;

// pub use contact_section::*;
pub use footer_section::*;
pub use header_section::*;
pub use hero_section::*;
pub use learnmore_section::*;
pub use screenplay_section::*;
pub use servicedetail_section::*;
pub use workprocess_section::*;
pub use workwithus_section::*;

use leptos::prelude::*;

#[component]
pub fn Section(children: Children) -> impl IntoView {
    view! { <section>{children()}</section> }
}

#[component]
pub fn FormInput(
    #[prop(into)] label: Signal<&'static str>,
    #[prop(into)] placeholder: Signal<&'static str>,
    #[prop(into)] input_type: Signal<InputType>,
    #[prop(into)] value: RwSignal<String>,
    #[prop(into, optional)] vertical: Option<RwSignal<bool>>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] required: MaybeProp<bool>,
) -> impl IntoView {
    let vertical = vertical.unwrap_or(RwSignal::new(true));

    view! {
        <div class=move || {
            if vertical.get() { "flex flex-col gap-2" } else { "flex gap-2" }
        }>
            <label class="font-medium">{label}</label>
            <input
                type=move || input_type.get().as_str()
                id=id
                name=id
                placeholder=placeholder
                required=required
                class="p-2 rounded-md border border-gray-300 focus:ring-2 focus:ring-stone-400"
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
            />
        </div>
    }
}

#[component]
pub fn FormTextArea(
    #[prop(into)] label: Signal<&'static str>,
    #[prop(into)] placeholder: Signal<&'static str>,
    #[prop(into)] value: RwSignal<String>,
    #[prop(into, optional)] vertical: Option<RwSignal<bool>>,
    #[prop(into, optional)] row: Option<MaybeProp<String>>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] required: MaybeProp<bool>,
) -> impl IntoView {
    let vertical = vertical.unwrap_or(RwSignal::new(true));
    let row = row.unwrap_or("6".into());

    view! {
        <div class=move || {
            if vertical.get() { "flex flex-col gap-2" } else { "flex gap-2" }
        }>
            <label class="font-medium">{label}</label>
            <textarea
                placeholder=placeholder
                id=id
                name=id
                required=required
                class="p-2 rounded-md border border-gray-300 focus:ring-2 focus:ring-stone-400"
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
                rows=row
            ></textarea>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Radio,
    Submit,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Radio => "radio",
            InputType::Submit => "submit",
            InputType::Password => "password",
        }
    }
}
