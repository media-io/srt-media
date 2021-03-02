use super::{
  filters::{AudioFilter, VideoFilter},
  graph,
};
use crate::{source::Decoder, Result};
use stainless_ffmpeg::{
  audio_decoder::AudioDecoder, format_context::FormatContext, video_decoder::VideoDecoder,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
pub struct StreamDescriptor {
  pub index: usize,
  descriptor: Descriptor,
}

impl StreamDescriptor {
  pub fn new_audio(index: usize, filters: Vec<AudioFilter>) -> Self {
    StreamDescriptor {
      index,
      descriptor: filters.into(),
    }
  }

  pub fn new_video(index: usize, filters: Vec<VideoFilter>) -> Self {
    StreamDescriptor {
      index,
      descriptor: filters.into(),
    }
  }

  pub fn new_data(index: usize) -> Self {
    let descriptor = Descriptor::DataDescriptor;
    StreamDescriptor { index, descriptor }
  }

  pub fn build_decoder(&self, format_context: Arc<Mutex<FormatContext>>) -> Result<Decoder> {
    match &self.descriptor {
      Descriptor::AudioDescriptor(audio_descriptor) => {
        // AudioDecoder can decode any codec, not only video
        let audio_decoder = AudioDecoder::new(
          format!("decoder_{}", self.index),
          &format_context.lock().unwrap(),
          self.index as isize,
        )?;

        let audio_graph =
          graph::build_audio_filter_graph(&audio_descriptor.filters, &audio_decoder)?;

        Ok(Decoder::new_audio_decoder(audio_decoder, audio_graph))
      }
      Descriptor::ImageDescriptor(video_descriptor) => {
        // VideoDecoder can decode any codec, not only video
        let video_decoder = VideoDecoder::new(
          format!("decoder_{}", self.index),
          &format_context.lock().unwrap(),
          self.index as isize,
        )?;

        let video_graph =
          graph::build_video_filter_graph(&video_descriptor.filters, &video_decoder)?;

        Ok(Decoder::new_video_decoder(video_decoder, video_graph))
      }
      _ => unimplemented!(),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Descriptor {
  AudioDescriptor(AudioDescriptor),
  ImageDescriptor(ImageDescriptor),
  DataDescriptor,
}

impl From<Vec<AudioFilter>> for Descriptor {
  fn from(filters: Vec<AudioFilter>) -> Self {
    let audio_descriptor = AudioDescriptor { filters };
    Descriptor::AudioDescriptor(audio_descriptor)
  }
}

impl From<Vec<VideoFilter>> for Descriptor {
  fn from(filters: Vec<VideoFilter>) -> Self {
    let image_descriptor = ImageDescriptor { filters };
    Descriptor::ImageDescriptor(image_descriptor)
  }
}

#[derive(Debug, PartialEq)]
pub struct AudioDescriptor {
  filters: Vec<AudioFilter>,
}

#[derive(Debug, PartialEq)]
pub struct ImageDescriptor {
  filters: Vec<VideoFilter>,
}
