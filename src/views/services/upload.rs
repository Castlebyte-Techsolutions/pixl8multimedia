use leptoaster::expect_toaster;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{FormData, HtmlFormElement, SubmitEvent};
use leptos_meta::Title;
use leptos_router::components::Form;
use leptos_icons::Icon;

#[component]
pub fn UploadMaterialsView() -> impl IntoView {
    view! {
		<Title text="Upload Materials" />

		<div class="container grid p-6 py-8 mx-auto mt-24 mb-12 bg-gray-100 rounded-lg shadow-lg sm:py-10 md:grid-cols-2 md:py-16 lg:py-24">
			<div class="flex flex-col gap-2">
				<h2 class="text-2xl font-semibold text-gray-900">
					"Submission Guidelines for Screenplay Adaptations"
				</h2>
				<p class="mt-2 text-lg text-gray-700">
					"At Pixl8Media, we are committed to bringing great stories to life. Please follow the guidelines below when submitting materials."
				</p>

				// <!-- Required Materials -->
				<div class="mt-4">
					<h3 class="text-lg font-semibold text-gray-800">
						"Required Materials"
					</h3>
					<ul class="pl-5 text-lg list-disc list-inside text-gray-700">
						<li>"Original Source Material (PDF, Word, ePub)"</li>
						<li>"Synopsis (One-page summary)"</li>
						<li>"Treatment (if available)"</li>
						<li>"Logline (One or two sentences)"</li>
						<li>"Rights & Permissions"</li>
					</ul>
				</div>

				// <!-- Formatting Requirements -->
				<div class="mt-4">
					<h3 class="text-lg font-semibold text-gray-800">
						"Formatting Requirements"
					</h3>
					<ul class="pl-5 text-lg list-disc list-inside text-gray-700">
						<li>
							"Screenplays in industry-standard format (Final Draft, Celtx, PDF)"
						</li>
						<li>
							"Text documents in a clear font (12pt, double-spaced)"
						</li>
						<li>"Optional visual references"</li>
					</ul>
				</div>

			</div>
			// <!-- User Information -->

			<FormUpload />
		</div>
	}
}

#[component]
fn FormUpload() -> impl IntoView {
    let toaster = expect_toaster();

    let is_loading = RwSignal::new(false);

    let email_val = RwSignal::new(String::new());
    let subject_val = RwSignal::new(String::new());
    let message_val = RwSignal::new(String::new());
    let fname_val = RwSignal::new(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let form_data = FormData::new_with_form(&target).unwrap();

        let toaster = toaster.clone();

        spawn_local(async move {
            is_loading.set(true);

            match api::send_attachment_req(form_data).await {
                Ok(_) => {
                    target.reset();
                    is_loading.set(false);

                    dispatch_toast(toaster, ToastPosition::TopEnd, ToastIntent::Success, formatted_date.clone().to_string(), "Your Material has been uploaded we will give you an email feedback once we review your materials".to_string(), "Materials uploaded successfully".to_string())
                }
                Err(ex) => {
                    leptos::logging::error!("{ex:?}");
                    is_loading.set(false);
                    dispatch_toast(
                        toaster,
                        ToastPosition::TopEnd,
                        ToastIntent::Error,
                        formatted_date.to_string(),
                        ex.to_string(),
                        "Upload failed".to_string(),
                    )
                }
            }
        });
    };

    view! {
		<div class="flex flex-col gap-3">
			<h3 class="text-lg font-bold text-gray-800 md:text-xl lg:text-2xl">
				"Submit Your Details"
			</h3>

			<Form attr:class="mt-4 space-y-4" action="" on:submit=on_submit>
				// <!-- Name -->
				<div>
					<label class="block font-medium text-gray-700">
						"Full Name *"
					</label>
					<input
						required=true
						name="email_fname"
						prop:value=move || fname_val.get()
						on:input=move |e| fname_val.set(event_target_value(&e))
						type="text"
						class="p-2 mt-1 w-full rounded-lg border"
						placeholder="Enter your full name"
					/>
				</div>

				// <!-- Email -->
				<div>
					<label class="block font-medium text-gray-700">
						"Email *"
					</label>
					<input
						required=true
						prop:value=move || email_val.get()
						on:input=move |e| email_val.set(event_target_value(&e))
						type="email"
						name="email_add"
						class="p-2 mt-1 w-full rounded-lg border"
						placeholder="Enter your email"
					/>
				</div>

				// <!-- Subject -->
				<div>
					<label class="block font-medium text-gray-700">
						"Subject *"
					</label>
					<input
						required=true
						prop:value=move || subject_val.get()
						on:input=move |e| subject_val.set(event_target_value(&e))
						type="text"
						name="email_subj"
						class="p-2 mt-1 w-full rounded-lg border"
						placeholder="Enter the subject"
					/>
				</div>

				// <!-- Message -->
				<div>
					<label class="block font-medium text-gray-700">
						"Message *"
					</label>
					<textarea
						required=true
						prop:value=move || message_val.get()
						on:input=move |e| message_val.set(event_target_value(&e))
						name="email_msg"
						class="p-2 mt-1 w-full rounded-lg border"
						rows="5"
						placeholder="Write your message here"
					></textarea>
				</div>

				// <!-- File Upload -->
				<div>
					<label class="block font-medium text-gray-700">
						"Upload Your Materials"
					</label>
					<label
						for="email_file"
						class="flex flex-col justify-center items-center p-6 w-full h-24 bg-transparent rounded-lg border-2 border-gray-300 border-dashed shadow-md cursor-pointer shadow-gray-200"
					>
						<Icon
							icon=icondata::TbCloudUp
							attr:class="text-gray-600 size-42"
						/>
						<span class="font-normal text-center text-gray-600">
							"Click to upload image"
						</span>
						<input
							required=true
							type="file"
							id="email_file"
							name="email_file"
							class="hidden"
						/>
					</label>
				</div>

				// <!-- Submit Button -->
				<Button
					button_type=ButtonType::Submit
					loading=is_loading
					icon=icondata::BsUpload
					appearance=ButtonAppearance::Primary
					size=ButtonSize::Large
				>
					"Submit Materials"
				</Button>
			</Form>
		</div>
	}
}

#[cfg(feature = "ssr")]
async fn read_file_bytes(file_path: &str) -> Result<Vec<u8>, ServerFnError>{
	let path = std::path::Path::new(file_path);
	let data = tokio::fs::read(path).await?;

	Ok(data)
}

