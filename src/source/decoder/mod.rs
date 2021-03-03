mod inner_decoder;

use crate::frame_data::FrameData;
use inner_decoder::InnerDecoder;
use stainless_ffmpeg::prelude::*;

pub struct Decoder {
  inner_decoder: InnerDecoder,
  graph: Option<FilterGraph>,
}

impl Decoder {
  pub fn new_audio_decoder(decoder: AudioDecoder, graph: Option<FilterGraph>) -> Self {
    let inner_decoder = InnerDecoder::AudioDecoder(decoder);
    Decoder {
      inner_decoder,
      graph,
    }
  }

  pub fn new_video_decoder(decoder: VideoDecoder, graph: Option<FilterGraph>) -> Self {
    let inner_decoder = InnerDecoder::VideoDecoder(decoder);
    Decoder {
      inner_decoder,
      graph,
    }
  }

  pub fn decode(&mut self, packet: &Packet) -> std::result::Result<Option<FrameData>, String> {
    match &self.inner_decoder {
      InnerDecoder::AudioDecoder(audio_decoder) => {
        unsafe {
          log::warn!(
            "[FFmpeg] Send packet to audio decoder {:?} {}",
            (*packet.packet).size,
            (*audio_decoder.codec_context).codec_id as i32
          );
        }
        let av_frame = unsafe {
          let ret_code = avcodec_send_packet(audio_decoder.codec_context, packet.packet);
          check_result!(ret_code);

          let av_frame = av_frame_alloc();
          let ret_code = avcodec_receive_frame(audio_decoder.codec_context, av_frame);
          check_result!(ret_code);

          let frame = Frame {
            frame: av_frame,
            name: Some("audio_source_1".to_string()),
            index: 1,
          };

          if let Some(graph) = &self.graph {
            if let Ok((audio_frames, _video_frames)) = graph.process(&[frame], &[]) {
              log::trace!("[FFmpeg] Output graph count {} frames", audio_frames.len());
              let frame = audio_frames.first().unwrap();
              av_frame_clone((*frame).frame)
            } else {
              av_frame
            }
          } else {
            av_frame
          }
        };

        let frame = Frame {
          frame: av_frame,
          name: Some("audio".to_string()),
          index: 1,
        };

        Ok(Some(FrameData::AudioVideo(frame)))
      }
      InnerDecoder::VideoDecoder(video_decoder) => {
        log::trace!("[FFmpeg] Send packet to video decoder");

        let av_frame = unsafe {
          let ret_code = avcodec_send_packet(video_decoder.codec_context, packet.packet);
          check_result!(ret_code);

          let av_frame = av_frame_alloc();
          let ret_code = avcodec_receive_frame(video_decoder.codec_context, av_frame);
          check_result!(ret_code);

          let frame = Frame {
            frame: av_frame,
            name: Some("video_source_1".to_string()),
            index: 1,
          };

          if let Some(graph) = &self.graph {
            if let Ok((_audio_frames, video_frames)) = graph.process(&[], &[frame]) {
              log::trace!("[FFmpeg] Output graph count {} frames", video_frames.len());
              let frame = video_frames.first().unwrap();
              av_frame_clone((*frame).frame)
            } else {
              av_frame
            }
          } else {
            av_frame
          }
        };

        let frame = Frame {
          frame: av_frame,
          name: Some("video".to_string()),
          index: 1,
        };

        Ok(Some(FrameData::AudioVideo(frame)))
      } // InnerDecoder::EbuTtmlLiveDecoder(ebu_ttml_live_decoder) => {
        //   let result = match ebu_ttml_live_decoder.decode(packet)? {
        //     Some(ttml_content) => Some(FrameData::EbuTtmlLive(Box::new(ttml_content))),
        //     None => None,
        //   };
        //   Ok(result)
        // }
    }
  }
}
