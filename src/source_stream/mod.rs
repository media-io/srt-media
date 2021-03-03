mod builder;
mod file_source_stream;
mod srt_source_stream;

pub use builder::SourceStreamBuilder;
pub use file_source_stream::FileSourceStream;
pub use srt_source_stream::SrtSourceStream;

use crate::Result;
use bytes::Bytes;
use std::time::Instant;

pub trait SourceStream {
  fn receive(&mut self) -> Option<(Instant, Bytes)>;
}
