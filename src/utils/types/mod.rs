mod email_type;
mod linktag_type;
mod video_type;

pub use email_type::*;
pub use linktag_type::*;
pub use video_type::VideoType;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResultReturn {
    pub state: String,
    pub success: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryReturnMessage {
    pub result: ResultReturn,
}
