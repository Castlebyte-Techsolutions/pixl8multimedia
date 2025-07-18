use leptos::{
    prelude::{FromServerFnError, ServerFnErrorErr},
    server_fn::codec::JsonEncoding,
};
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum AppError {
    DatabaseError(String),
    FieldEmailRequired(String),
    ErrorUploadFile(String),
    ErrorFileRead(String),
    InvalidFieldName,
    IOError(String),
    EmailPortParseError(String),
    SendingEmailFailed(String),
    SmtpConnectServerFailed(String),

    // For leptos server fn error
    ServerFnError(ServerFnErrorErr),

    // Other utils error
    #[cfg(feature = "ssr")]
    EnvsError(String),
    #[cfg(feature = "ssr")]
    B64uError(String),
    #[cfg(feature = "ssr")]
    B58Error(String),
    #[cfg(feature = "ssr")]
    TimeError(String),
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AppError::ServerFnError(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IOError(value.to_string())
    }
}
#[cfg(feature = "ssr")]
impl From<crate::utils::b64::Error> for AppError {
    fn from(value: crate::utils::b64::Error) -> Self {
        AppError::B64uError(value.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<crate::utils::envs::Error> for AppError {
    fn from(value: crate::utils::envs::Error) -> Self {
        AppError::EnvsError(value.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<crate::utils::b58::Error> for AppError {
    fn from(value: crate::utils::b58::Error) -> Self {
        AppError::B58Error(value.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<crate::utils::time::Error> for AppError {
    fn from(value: crate::utils::time::Error) -> Self {
        AppError::TimeError(value.to_string())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmailPortParseError(err) => write!(f, "SMTP port parsing error {err:?}"),
            Self::SmtpConnectServerFailed(err) => write!(f, "SMTP port parsing error {err:?}"),
            Self::SendingEmailFailed(err) => write!(f, "SMTP port parsing error {err:?}"),
            Self::ErrorFileRead(err) => write!(f, "Error reading file {err:?}"),
            Self::IOError(err) => write!(f, "IO Error {err:?}"),
            Self::DatabaseError(err) => write!(f, "Database error: {err:?}"),
            Self::FieldEmailRequired(err) => write!(f, "Field email required {err:?}"),
            Self::InvalidFieldName => write!(f, "Invalid field name"),
            Self::ErrorUploadFile(err) => write!(f, "Error upload file {err:?}"),
            Self::ServerFnError(err) => write!(f, "Server Fn Error: {err:?}"),

            #[cfg(feature = "ssr")]
            Self::EnvsError(err) => write!(f, "ENV error: {err:?}"),
            #[cfg(feature = "ssr")]
            Self::B64uError(err) => write!(f, "B64U error: {err:?}"),
            #[cfg(feature = "ssr")]
            Self::B58Error(err) => write!(f, "B58 error: {err:?}"),
            #[cfg(feature = "ssr")]
            Self::TimeError(err) => write!(f, "Time error: {err:?}"),
        }
    }
}
