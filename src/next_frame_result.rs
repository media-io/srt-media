use crate::frame_data::FrameData;
use std::fmt;

pub enum NextFrameResult {
  EndOfStream,
  Frame {
    stream_index: usize,
    frame: FrameData,
  },
  Nothing,
  WaitMore,
}

impl fmt::Display for NextFrameResult {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      NextFrameResult::EndOfStream => write!(fmt, "End of stream"),
      NextFrameResult::Frame { stream_index, .. } => {
        write!(fmt, "Frame for stream {}", stream_index)
      }
      NextFrameResult::Nothing => write!(fmt, "Nothing"),
      NextFrameResult::WaitMore => write!(fmt, "Wait more"),
    }
  }
}
