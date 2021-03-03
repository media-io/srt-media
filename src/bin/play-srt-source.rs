use cpal::{SampleRate, Stream};
use ringbuf::{Consumer, RingBuffer};
use srt_media::{
  source::{
    filters::{AudioFilter, AudioFormat},
    SrtSource, StreamDescriptor,
  },
  FrameData, NextFrameResult,
};
use stainless_ffmpeg::prelude::*;

const SAMPLE_RATE: SampleRate = SampleRate(48_000);

fn main() {
  pretty_env_logger::init();

  let url = std::env::args().last().unwrap();
  let mut srt_source = SrtSource::new(&url);

  let nb_stream = srt_source.format_context.lock().unwrap().get_nb_streams();

  let mut first_audio_stream = None;

  for i in 0..nb_stream {
    let stream_type = srt_source
      .format_context
      .lock()
      .unwrap()
      .get_stream_type(i as isize);

    log::info!("Stream {}: {:?}", i, stream_type);

    if stream_type == AVMediaType::AVMEDIA_TYPE_AUDIO {
      first_audio_stream = Some(i);
    }
  }

  let first_audio_stream = first_audio_stream.unwrap();

  let channel_layouts = vec!["stereo".to_string()];
  let sample_formats = vec!["s32".to_string()];
  let sample_rates = vec![48000];

  let audio_filters = vec![AudioFilter::Format(AudioFormat {
    sample_rates,
    channel_layouts,
    sample_formats,
  })];

  let selection = vec![StreamDescriptor::new_audio(
    first_audio_stream as usize,
    audio_filters,
  )];

  srt_source.select_streams(selection).unwrap();

  let (mut producer, consumer) = RingBuffer::<f32>::new(1024 * 1024).split();

  let _stream = audio_player(consumer);

  loop {
    let next_frame = srt_source.next_frame().unwrap();
    match &next_frame {
      NextFrameResult::Nothing | NextFrameResult::WaitMore => {
        std::thread::sleep(std::time::Duration::from_millis(10));
      }
      NextFrameResult::Frame { frame, .. } => {
        if let FrameData::AudioVideo(av_frame) = frame {
          unsafe {
            let av_frame = av_frame.frame;

            let size = ((*av_frame).channels * (*av_frame).nb_samples) as usize;

            log::info!(
              "Frame {} samples, {} channels, {} bytes // {} bytes",
              (*av_frame).nb_samples,
              (*av_frame).channels,
              (*av_frame).linesize[0],
              size,
            );

            let samples: Vec<i32> = Vec::from_raw_parts((*av_frame).data[0] as _, size, size);

            let float_samples: Vec<f32> = samples
              .iter()
              .map(|value| (*value as f32) / i32::MAX as f32)
              .collect();

            producer.push_slice(&float_samples);
            std::mem::forget(samples);
          }
        }
      }
      _ => {}
    }
    std::mem::forget(next_frame);
  }
}

fn audio_player(mut consumer: Consumer<f32>) -> Stream {
  use cpal::traits::{DeviceTrait, HostTrait};

  let host = cpal::default_host();
  let device = host
    .default_output_device()
    .expect("no output device available");

  let mut supported_configs_range = device
    .supported_output_configs()
    .expect("error while querying configs");

  let supported_config = supported_configs_range
    .find(|config| {
      config.channels() == 2
        && SAMPLE_RATE >= config.min_sample_rate()
        && SAMPLE_RATE <= config.max_sample_rate()
    })
    .expect("no supported config?!")
    .with_sample_rate(SAMPLE_RATE);

  let config = supported_config.into();

  device
    .build_output_stream(
      &config,
      move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for i in 0..data.len() {
          data[i] = 0.0;
        }
        if consumer.len() > data.len() {
          consumer.pop_slice(data);
        }
      },
      move |err| log::error!("CPAL error: {:?}", err),
    )
    .unwrap()
}
