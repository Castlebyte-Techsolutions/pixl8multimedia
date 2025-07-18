use leptoaster::{expect_toaster, ToastBuilder};
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_icons::Icon;
use leptos_meta::Title;
use leptos_router::components::Form;
use serde::{Deserialize, Serialize};

use crate::sections::{FormInput, FormTextArea, InputType};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmailCredentials {
    email_add: String,
    email_subj: String,
    email_msg: String,
}
use crate::error::Result;

#[tracing::instrument]
#[server]
pub async fn email_send_api(
    email_add: String,
    email_subj: String,
    email_msg: String,
) -> Result<String> {
    use crate::error::AppError;
use crate::utils::configs::email_config;

    let (smtp_user, smtp_pass, smtp_host, smtp_port, _email_to, _email_to_dev) = (
        &email_config().GMAIL_SMTP_USER,
        &email_config().GMAIL_SMTP_PASS,
        &email_config().GMAIL_SMTP_HOST,
        &email_config().GMAIL_SMTP_PORT,
        &email_config().IONOS_USER,
        &email_config().EMAIL_SEND_DEV,
    );

    let template =
        crate::templates::email_template::template_01(email_add, email_subj.clone(), email_msg);
    let message = mail_send::mail_builder::MessageBuilder::new()
        .from((smtp_user.as_str(), "Pixl8Multimedia Support"))
        .to(_email_to.as_str())
        .subject(email_subj.as_str())
        .html_body(template);

    let client_res = mail_send::SmtpClientBuilder::new(
        smtp_host.as_str(),
        smtp_port.parse()
            .map_err(|_| AppError::EmailPortParseError("Unable to parse smtp port".into()))?,
    )
    .implicit_tls(false)
    .credentials((smtp_user.as_str(), smtp_pass.as_str()))
    .connect()
    .await;

    match client_res {
        Ok(mut client) => {
            if let Err(e) = client.send(message).await {
                tracing::error!("Email sent failed: {e:?}");
                return Err(AppError::ServerFnError(ServerFnErrorErr::Response("Email sent failed".to_string())));

            }
            Ok("Email has been send successfully, We'll get you in touch soon".into())
        }
        Err(e) => {
            tracing::error!("Something went wrong with email");
            Err(AppError::ServerFnError(ServerFnErrorErr::ServerError(e.to_string())))
        }
    }
}

#[component]
pub fn ContactUsView() -> impl IntoView {
    let toaster = expect_toaster();

    let is_loading = RwSignal::new(false);

    let email_val = RwSignal::new(String::new());
    let subject_val = RwSignal::new(String::new());
    let message_val = RwSignal::new(String::new());

    let submit_form = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let toaster = toaster.clone();

        if !email_val.get().is_empty()
            && !subject_val.get().is_empty()
            && !message_val.get().is_empty()
        {
            is_loading.set(true);
            spawn_local(async move {
                let eval = move || email_val.get();
                let sval = move || subject_val.get();
                let mval = move || message_val.get();

                match email_send_api(eval(), sval(), mval()).await {
                    Ok(returned_val) => {
                        email_val.set(String::new());
                        subject_val.set(String::new());
                        message_val.set(String::new());

                        is_loading.set(false);

                        toaster.toast(
                            ToastBuilder::new(returned_val)
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
        }
    };

    view! {
		<Title text="Contact Us" />

		<div class="py-12 mx-auto mt-24 mb-12 w-full h-full md:w-1/2">
			<div class="flex flex-col-reverse items-center w-full h-full lg:flex-row border-zinc-500">
				<div class="w-full text-white bg-zinc-950 lg:w-[340px]">
					<div class="flex flex-col gap-6 py-10 px-5">
						<h1 class="text-lg font-normal uppercase">"contact"</h1>
						<h1 class="text-4xl font-bold capitalize">
							"get in " <span class="text-blue-600">"touch"</span>
						</h1>
						<div class="flex-col font-normal duration-300 ease-in-out hover:text-blue-500 flext">
							<span class="text-sm font-normal">"Melbourne Florida"</span>
							<p class="text-sm font-normal">
								"Opening : Mon-Fri 9:00 am – 6:00 pm EST"
							</p>
						</div>
						<div class="flex gap-5 items-center">
							<Icon
								icon=icondata::BsFacebook
								width="1.5rem"
								height="1.5rem"
							/>
							<span>"Pixl8Multimedia page"</span>
						</div>
						<div class="flex gap-5 items-center">
							<Icon
								icon=icondata::BsMessenger
								width="1.5rem"
								height="1.5rem"
							/>
							<span>"Pixl8Multimedia messenger"</span>
						</div>
						<div class="flex gap-5 items-center">
							<Icon
								icon=icondata::BsTelephoneFill
								width="1.5rem"
								height="1.5rem"
							/>
							<span>"(+1)312 212 3899"</span>
						</div>
					</div>
				</div>

				<Form
					action=""
					on:submit=submit_form
					attr:class="flex flex-col flex-col gap-5 w-full p-5"
				>

					<FormInput
						label="Email *"
						required=true
						id="email"
						input_type=InputType::Email
						placeholder="email@example.com"
						value=email_val
					/>

					<FormInput
						label="Subject *"
						id="subject"
						required=true
						input_type=InputType::Text
						placeholder="let us know how we can help you."
						value=subject_val
					/>

					<FormTextArea
						label="Your message."
						id="message"
						required=true
						placeholder="Leave a comment..."
						value=message_val
						row="7"
					/>

					<div class="flex items-center">

						<div class="mx-auto"></div>

						<button
							type="submit"
							disabled=is_loading
							class="flex gap-2 items-center py-2 px-6 mt-2 bg-transparent rounded-md border transition ease-in cursor-pointer hover:text-white duratio-300 border-zinc-500 hover:text-stone-800 hover:bg-zinc-800"
						>
							"SEND MESSAGE"
							<Icon icon=icondata::BsSendFill attr:class="h-4 w-4" />
						</button>
					</div>

				</Form>
			</div>
		</div>
	}
}
