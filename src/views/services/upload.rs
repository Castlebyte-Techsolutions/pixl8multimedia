use leptoaster::{expect_toaster, ToastBuilder};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{FormData, HtmlFormElement, SubmitEvent};
use leptos_meta::Title;
use leptos_router::components::Form;
use phosphor_leptos::{Icon, ARROW_CLOCKWISE, CLOUD_ARROW_DOWN};
use server_fn::codec::{MultipartData, MultipartFormData};

use crate::error::Result;

#[tracing::instrument]
#[server(input = MultipartFormData)]
pub async fn email_send_with_attachments(multipart: MultipartData) -> Result<String> {
    use crate::error::AppError;
    use crate::templates;
    use crate::utils::b58;
    use crate::utils::configs::email_config;
    use crate::utils::types::email_type::EmailSendWithAttachment;
    use chrono::{DateTime, Local};
    use std::io::Write;
    use std::path::Path;
    use tokio::fs;

    let mut multipart = multipart.into_inner().unwrap();
    let (smtp_user, smtp_pass, smtp_host, smtp_port, _email_to, _email_to_dev) = (
        &email_config().GMAIL_SMTP_USER,
        &email_config().GMAIL_SMTP_PASS,
        &email_config().GMAIL_SMTP_HOST,
        &email_config().GMAIL_SMTP_PORT,
        &email_config().IONOS_USER,
        &email_config().EMAIL_SEND_DEV,
    );

    let mut email_data = EmailSendWithAttachment::new();
    let v7_uuid = uuid::Uuid::now_v7();
    let new_uuid = b58::b58(v7_uuid);

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| AppError::ErrorUploadFile(err.to_string()))?
    {
        let name = field.name().map(|s| s.to_string()).unwrap_or_default();

        match name.as_str() {
            "email_fname" => email_data.email_fname = field.text().await.unwrap_or_default(),
            "email_add" => email_data.email_add = field.text().await.unwrap_or_default(),
            "email_subj" => email_data.email_subj = field.text().await.unwrap_or_default(),
            "email_msg" => email_data.email_msg = field.text().await.unwrap_or_default(),
            "email_file" => {
                let file_name = field.file_name().unwrap().to_string();
                let mime_type = field.content_type().unwrap().to_string();
                let data = field.bytes().await.unwrap();

                let temp_01 = file_name.split('.').collect::<Vec<_>>();
                let now: DateTime<Local> = Local::now();
                let formatted_date = now.format("%Y-%m-%d").to_string();
                let new_file_name = format!(
                    "{}-{}-{}.{}",
                    new_uuid,
                    formatted_date,
                    email_data.email_fname.clone().to_lowercase(),
                    temp_01.get(1).unwrap()
                );

                let save_path = format!("uploads/{}", new_file_name.clone());
                let path = Path::new(&save_path);

                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).await.unwrap();
                }

                let mut file = std::fs::File::create(path).unwrap();
                file.write_all(&data).unwrap();

                email_data.file_path = save_path.clone();
                email_data.file_name = new_file_name;
                email_data.mime_type = mime_type;
            }
            _ => {
                return Err(AppError::InvalidFieldName);
            }
        }
    }

    let EmailSendWithAttachment {
        email_fname,
        email_add,
        email_subj,
        email_msg,
        mime_type,
        file_name,
        file_path,
    } = email_data;

    if email_add.is_empty() || email_subj.is_empty() {
        return Err(AppError::FieldEmailRequired(
            "Email address and Subject is required".to_string(),
        ));
    }

    let template = templates::email_template::template_02(
        email_fname,
        email_add,
        email_subj.clone(),
        email_msg,
    );

    let file_bytes = match read_file_bytes(&file_path).await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Reading file bytes error: {}", e.to_string());
            return Err(AppError::ErrorFileRead(e.to_string()));
        }
    };

    let message = mail_send::mail_builder::MessageBuilder::new()
        .from((smtp_user.as_str(), "Pixl8Multimedia Support"))
        .to(_email_to.as_str())
        .subject(email_subj)
        .html_body(template)
        .attachment(mime_type.clone(), file_name.clone(), file_bytes);
    let client_res = mail_send::SmtpClientBuilder::new(
        smtp_host.as_str(),
        smtp_port
            .parse()
            .map_err(|_| AppError::EmailPortParseError("Unable to parse smtp port".into()))?,
    )
    .implicit_tls(false)
    .credentials((smtp_user.as_str(), smtp_pass.as_str()))
    .connect()
    .await;

    match client_res {
        Ok(mut client) => {
            if let Err(e) = client.send(message).await {
                tracing::error!("{:<20} - {:?}", "upload send email", e);
                return Err(AppError::SendingEmailFailed(e.to_string()));
            }

            std::fs::remove_file(file_path)?;

            Ok("Materials sent successfully we'll get you in touch soon".to_string())
        }
        Err(e) => {
            tracing::error!("{:<20} - {:?}", "uploading error", e);
            Err(AppError::SmtpConnectServerFailed(e.to_string()))
        }
    }
}

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

            match email_send_with_attachments(form_data.into()).await {
                Ok(returned_val) => {
                    target.reset();
                    is_loading.set(false);
                    toaster.toast(
                        ToastBuilder::new(returned_val.to_string())
                            .with_level(leptoaster::ToastLevel::Success)
                            .with_expiry(Some(4_500))
                            .with_position(leptoaster::ToastPosition::TopRight),
                    );
                }
                Err(ex) => {
                    leptos::logging::error!("{ex:?}");
                    is_loading.set(false);
                    toaster.toast(
                        ToastBuilder::new(ex.to_string())
                            .with_level(leptoaster::ToastLevel::Error)
                            .with_expiry(Some(4_500))
                            .with_position(leptoaster::ToastPosition::TopRight),
                    );
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
							icon=CLOUD_ARROW_DOWN
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
				<Show
					when=move || !is_loading.get()
					fallback=|| {
						view! {
							<button
								type="button"
								disabled
								class="flex justify-center items-center py-2 w-full rounded cursor-progress bg-stone-500/20 text-stone-600"
							>
								<Icon
									icon=ARROW_CLOCKWISE
									attr:class="h-6 w-6 animate-spin"
								/>
							</button>
						}
					}
				>
					<button
						type="submit"
						class="py-2 w-full rounded duration-200 ease-in-out cursor-pointer hover:scale-95 bg-sky-500 text-stone-50"
					>
						"Submit"
					</button>
				</Show>
			</Form>
		</div>
	}
}

#[cfg(feature = "ssr")]
async fn read_file_bytes(file_path: &str) -> Result<Vec<u8>> {
    let path = std::path::Path::new(file_path);
    let data = tokio::fs::read(path).await?;

    Ok(data)
}
