use serde::{Deserialize, Serialize};

use super::b64::b64u_decode;
use std::{env, str::FromStr};

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::MissingEnv(name.to_string()))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>()
        .map_err(|_| Error::WrongFormat(name.to_string()))
}

pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name.to_string()))
}

// region: --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum Error {
    MissingEnv(String),
    WrongFormat(String),
}

// region: --- Error boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Self::MissingEnv(err) => write!(fmt, "Missing variable in .env {err:?}"),
            Self::WrongFormat(err) => write!(fmt, "ENV variable wrong format {err:?}"),
        }
    }
}

// endregion: --- Error boilerplate

// endregion: --- Error
