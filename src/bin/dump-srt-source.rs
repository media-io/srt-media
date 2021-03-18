use srt_media::{
  source::{SrtSource, StreamDescriptor},
  NextFrameResult,
};
use stainless_ffmpeg::prelude::*;

fn main() {
  pretty_env_logger::init();

  let mut srt_source = SrtSource::new("srt://127.0.0.1:3333");
  // let mut srt_source = SrtSource::new("srt://194.51.35.43:8998");

  let nb_stream = srt_source.format_context.lock().unwrap().get_nb_streams();

  let mut first_video_stream = None;
  let mut first_audio_stream = None;

  for i in 0..nb_stream {
    let stream_type = srt_source
      .format_context
      .lock()
      .unwrap()
      .get_stream_type(i as isize);

    log::info!("Streams: {:?}", stream_type);

    if stream_type == AVMediaType::AVMEDIA_TYPE_VIDEO {
      first_video_stream = Some(i);
    }
    if stream_type == AVMediaType::AVMEDIA_TYPE_AUDIO {
      first_audio_stream = Some(i);
    }
  }

  let first_video_stream = first_video_stream.unwrap();
  let first_audio_stream = first_audio_stream.unwrap();

  let video_filters = vec![];
  let audio_filters = vec![];

  let selection = vec![
    StreamDescriptor::new_video(first_video_stream as usize, video_filters),
    StreamDescriptor::new_audio(first_audio_stream as usize, audio_filters),
  ];

  srt_source.select_streams(selection).unwrap();

  loop {
    let next_frame = srt_source.next_frame().unwrap();
    match next_frame {
      NextFrameResult::WaitMore | NextFrameResult::Nothing => {}
      _ => {
        println!("{}", next_frame);
      }
    }
    std::mem::forget(next_frame);
  }
}
