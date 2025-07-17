use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailSend {
    pub email_add: String,
    pub email_subj: String,
    pub email_msg: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailSendWrapper {
    pub email_data: EmailSend,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailAttachmentSend {
    pub email_fname: String,
    pub email_add: String,
    pub email_subj: String,
    pub email_msg: String,
}
