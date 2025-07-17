#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonType {
    #[default]
    Primary,
    Secondary,
}

#[derive(Debug, Clone, Default)]
pub enum ButtonType {
    Submit,
    Button,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Submit => "submit",
            ButtonType::Button => "button",
        }
    }
}

use leptos::prelude::*;

#[component]
fn Button(
    #[prop(optional, into)] loading: Option<bool>,
    #[prop(into, optional)] button_type: MaybeProp<ButtonType>,
) -> impl IntoView {
    view! {}
}
