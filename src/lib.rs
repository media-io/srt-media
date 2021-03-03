#[macro_use]
extern crate serde_derive;

mod error;
mod frame_data;
mod next_frame_result;

pub mod source;
pub mod source_stream;

pub use error::{Error, Result};
pub use frame_data::FrameData;
pub use next_frame_result::NextFrameResult;
