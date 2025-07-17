use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn RefundPolicyView() -> impl IntoView {
    view! {
        <Title text="Refund Policy" />

        <div class="max-w-5xl mx-auto p-6 bg-white shadow-lg rounded-lg mt-32 mb-12">
            <h1 class="text-4xl font-bold text-gray-800 border-b pb-2 mb-4">
                "Refund Policy for Pixl8Multimedia"
            </h1>

            <p class="mb-4 text-gray-700">
                "If a client decides not to move forward with the project and has signed a contract but has not paid the invoice, then the invoice does not need to be paid. After ten days, the contract is canceled, and the invoice will be voided."
            </p>

            <p class="mb-4 text-gray-700">
                "If a client decides to cancel after paying the invoice for a full refund, this request must be made within 3 days of payment."
            </p>

            <p class="mb-4 text-gray-700">
                "If a client decides to cancel after the 3-day period following payment, the client will be required to pay for any work that has been completed, plus 20% of the contract cost. This is stated in your contract. This amount will be deducted from the payment, and the remaining balance will be refunded."
            </p>

            <p class="mb-4 font-bold text-red-600">
                "ALL REFUNDS MUST BE REQUESTED THROUGH OUR FINANCE DEPARTMENT AND NOT VIA THE INVOICE PAYMENT SERVICE."
            </p>
            <p class="mb-4 text-gray-700">
                "THIS CAN BE DONE BY EMAILING YOUR REQUEST DIRECTLY TO: "
                <A
                    href="mailto:info@pixl8media.com"
                    attr:class="font-semibold text-blue-600 hover:underline"
                >
                    "CEO.PIxl8MEDIA@GMAIL.COM"
                </A> " WITH \"REFUND\" IN THE SUBJECT LINE."
            </p>

            <p class="mb-4 text-gray-700">
                "If you are considering a refund because you are unhappy with the draft product, please contact your Agent or Chief of Operations. Under your contract, you have free revision rights. It is our mission to provide you with a quality product that meets your goals."
            </p>
        </div>
    }
}
