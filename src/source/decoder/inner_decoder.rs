use stainless_ffmpeg::{audio_decoder::AudioDecoder, video_decoder::VideoDecoder};

pub enum InnerDecoder {
  AudioDecoder(AudioDecoder),
  VideoDecoder(VideoDecoder),
  // EbuTtmlLiveDecoder(EbuTtmlLiveDecoder),
}
