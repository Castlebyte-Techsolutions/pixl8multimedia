use web_sys::FormData;

use crate::{
    error::Error,
    utils::types::{EmailSendWrapper, QueryReturnMessage},
};

use super::{request_form_post, request_post};

pub async fn send_email_req(data: EmailSendWrapper) -> Result<QueryReturnMessage, Error> {
    request_post::<EmailSendWrapper, QueryReturnMessage>("/email/send".to_string(), data).await
}

pub async fn send_attachment_req(form_data: FormData) -> Result<QueryReturnMessage, Error> {
    request_form_post::<QueryReturnMessage>("/email/send/attachment".into(), form_data).await
}
