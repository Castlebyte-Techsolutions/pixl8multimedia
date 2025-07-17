use leptos::prelude::*;

#[component]
pub fn VideoEmbed(
    #[prop(into)] embed: Signal<String>,
    #[prop(into)] indx: Signal<usize>,
) -> impl IntoView {
    Effect::new(move |_| {
        let document = window().document().expect("Failed to get document");
        let parser = web_sys::DomParser::new().expect("Failed to create DOMParser");

        if let Ok(parsed) = parser.parse_from_string(&embed.get(), web_sys::SupportedType::TextHtml)
        {
            if let Some(embed) = parsed.query_selector("iframe").ok().flatten() {
                embed.remove_attribute("height").ok();
                embed.remove_attribute("width").ok();
                embed.class("h-full w-full");

                let container_id = format!("vid-{}", indx.get());
                if let Some(container) = document.get_element_by_id(&container_id) {
                    container.append_child(&embed).ok();
                }
            }
        }
    });

    view! {
        <div
            id=move || format!("vid-{}", indx.get())
            class="aspect-w-16 aspect-h-16 h-[500px] w-full"
        ></div>
    }
}
