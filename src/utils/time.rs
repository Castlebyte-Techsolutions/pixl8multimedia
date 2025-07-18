use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap()
    // TODO: need to check if time is safe
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::FailToDateParse(moment.to_string()))
}

// region: --- Error

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum Error {
    FailToDateParse(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Self::FailToDateParse(err) => write!(fmt, "Fail to date parse {err:?}"),
        }
    }
}
// endregion: --- Error boilerplate

// endregion: --- Error
