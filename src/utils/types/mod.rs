#[cfg(feature = "ssr")]
pub mod email_type;

mod linktag_type;
mod video_type;

pub use linktag_type::*;
pub use video_type::*;
