#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailSendCredentials {
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_host: String,
    pub smtp_port: String,
    pub email_to: String,
}

pub static EMAIL_CREDS: std::sync::OnceLock<EmailSendCredentials> = std::sync::OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EmailSendWithAttachment {
    pub email_fname: String,
    pub email_add: String,
    pub email_subj: String,
    pub email_msg: String,
    pub mime_type: String,
    pub file_name: String,
    pub file_path: String,
}

impl EmailSendWithAttachment {
    pub fn new() -> EmailSendWithAttachment {
        EmailSendWithAttachment {
            email_fname: String::new(),
            email_add: String::new(),
            email_subj: String::new(),
            email_msg: String::new(),
            mime_type: String::new(),
            file_name: String::new(),
            file_path: String::new(),
        }
    }
}
