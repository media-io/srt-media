// use crate::message::media::ebu_ttml_live::EbuTtmlLive;
use stainless_ffmpeg::frame::Frame;

pub enum FrameData {
  AudioVideo(Frame),
  // EbuTtmlLive(Box<EbuTtmlLive>),
  Data(Vec<u8>),
}

impl FrameData {
  pub fn get_pts(&self) -> i64 {
    match self {
      FrameData::AudioVideo(frame) => frame.get_pts(),
      // TODO: support pts for EbuTtmlLive
      // FrameData::EbuTtmlLive(_) |
      FrameData::Data(_) => 0,
    }
  }
}
