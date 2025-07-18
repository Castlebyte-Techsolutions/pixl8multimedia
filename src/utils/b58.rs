use base58::{FromBase58, ToBase58};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn b58(content: Uuid) -> String {
    content.as_bytes().to_base58()
}

pub fn b58_encoding(content: Uuid) -> Result<Vec<u8>> {
    let b58 = b58(content);
    b58.from_base58().map_err(|_| Error::FailToB58Encoding)
}

// region: --- Error

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum Error {
    FailToB58Encoding,
    Base58Error(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Self::FailToB58Encoding => write!(fmt, "Fail to B58 encoding"),
            Self::Base58Error(err) => write!(fmt, "Base 58 error encode: {err:?}"),
        }
    }
}

// endregion: --- Error boilerplate

// endregion: --- Error
