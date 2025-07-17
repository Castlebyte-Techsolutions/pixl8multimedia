use chrono::prelude::*;
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_router::components::Form;

use crate::utils::types::{EmailSend, EmailSendWrapper};

#[component]
pub fn ContactUsSection() -> impl IntoView {
    let email_val = RwSignal::new(String::new());
    let subject_val = RwSignal::new(String::new());
    let message_val = RwSignal::new(String::new());

    let is_loading = RwSignal::new(false);

    fn dispatch_toast(
        toaster: ToasterInjection,
        position: ToastPosition,
        intent: ToastIntent,
        cur_date: String,
        message: String,
        title: String,
    ) {
        toaster.dispatch_toast(
            move || {
                view! {
					<Toast>
						<ToastTitle>{title}</ToastTitle>
						<ToastBody>{message}</ToastBody>
						<ToastFooter>{cur_date}</ToastFooter>
					</Toast>
				}
            },
            ToastOptions::default()
                .with_intent(intent)
                .with_position(position),
        );
    }

    let submit_form = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let now: DateTime<Local> = Local::now();
        let formatted_date = now.format("%Y-%m-%d").to_string();

        if !email_val.get().is_empty()
            && !subject_val.get().is_empty()
            && !message_val.get().is_empty()
        {
            is_loading.set(true);
            spawn_local(async move {
                match send_email_request(email_val.get(), subject_val.get(), message_val.get())
                    .await
                {
                    Ok(_) => {
                        email_val.set(String::new());
                        subject_val.set(String::new());
                        message_val.set(String::new());

                        is_loading.set(false);

                        dispatch_toast(
                            toaster,
                            ToastPosition::TopEnd,
                            ToastIntent::Success,
                            formatted_date.clone().to_string(),
                            "Your message has been sent successfully we'll get you in touch"
                                .to_string(),
                            "Email sent successfully".to_string(),
                        );
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
                            "Email sent failed".to_string(),
                        );
                    }
                }
            });
        }
    };

    view! {
		<Section attr:class="relative w-full h-[800px] flex items-center justify-center bg-cover bg-center bg-no-repeat bg-[url(/images/working-group.jpg)]">
			<div class="absolute inset-0 bg-black/50"></div>

			<div class="flex relative flex-col items-center px-6 mx-auto w-full max-w-6xl md:flex-row">
				<div class="hidden flex-1 md:flex"></div>

				<div class="p-8 w-full rounded-lg shadow-lg md:w-1/2 bg-gray-50/80 text-stone-600">
					<h2 class="mb-4 text-2xl font-bold uppercase md:text-3xl">
						"LET'S CHAT OVER A BREW, COFFEE, TEA OR SOMETHING STRONGER..."
					</h2>

					<Form
						action=""
						on:submit=submit_form
						attr:class="flex flex-col gap-4"
					>

						<FormInput
							label="Email *"
							id="email"
							input_type=InputType::Email
							placeholder="email@example.com"
							value=email_val
						/>

						<FormInput
							label="Subject *"
							id="subject"
							input_type=InputType::Text
							placeholder="let us know how we can help you."
							value=subject_val
						/>

						<FormTextArea
							label="Your message."
							id="message"
							placeholder="Leave a comment..."
							value=message_val
						/>

						<div class="flex items-center">
							<Button
								button_type=ButtonType::Submit
								loading=is_loading
								class="flex gap-2 items-center py-2 px-6 mt-2 bg-transparent rounded-md border-2 border-white transition cursor-pointer hover:bg-white hover:text-stone-800"
							>
								"SEND MESSAGE"
								<Icon icon=icondata::BsSendFill class="w-4 h-4" />
							</Button>
						</div>
					</Form>
				</div>
			</div>
		</Section>
	}
}
