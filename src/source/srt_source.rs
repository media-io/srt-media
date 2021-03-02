use super::{decoder::Decoder, media_stream::MediaStream, stream_descriptor::StreamDescriptor};
use crate::{
  connection::Connection, next_frame_result::NextFrameResult, Error,
  Result,
};
use bytes::Buf;
use ringbuf::RingBuffer;
use stainless_ffmpeg::format_context::FormatContext;
use std::{
  collections::HashMap,
  io::Cursor,
  sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
  },
  thread::JoinHandle,
};

type SharedFormatContext = Arc<Mutex<FormatContext>>;
type AsyncChannelSenderReceiver = (Sender<SharedFormatContext>, Receiver<SharedFormatContext>);

pub struct SrtSource {
  _thread: JoinHandle<()>,
  pub format_context: Arc<Mutex<FormatContext>>,
  decoders: HashMap<usize, Decoder>,
}

impl SrtSource {
  // pub fn new() -> Self {
  // }

  pub fn new(source_url: &str) -> Self {
    log::info!("Opening source: {}", source_url);

    let (tx, rx): AsyncChannelSenderReceiver = mpsc::channel();
    let cloned_source_url = source_url.to_string();

    let thread = std::thread::spawn(move || {
      let mut connection = Connection::open_connection(&cloned_source_url).unwrap();

      let ring_buffer = RingBuffer::<u8>::new(100 * 1024 * 1024);
      let (mut producer, consumer) = ring_buffer.split();

      let (_instant, bytes) = connection
        .receive()
        .expect("Could not get the first bytes from SRT stream.");

      let size = bytes.len();
      log::debug!("Get first {} bytes to define stream format.", size);

      log::trace!("First {} bytes of the SRT stream: {:?}", size, bytes);
      let mut cursor = Cursor::new(bytes);
      let first_byte = cursor.get_u8();

      cursor.set_position(0);
      producer.read_from(&mut cursor, Some(size)).unwrap();

      let (format, threshold) = if first_byte == 0x47 {
        ("mpegts", 1024 * 1024)
      } else {
        ("data", 0)
      };

      let media_stream = MediaStream::new(format, consumer).unwrap();
      log::debug!(
        "Initializing media stream with format {:?}: {:?}",
        format,
        media_stream
      );

      let mut got_stream_info = false;

      while let Some((_instant, bytes)) = connection.receive() {
        log::trace!("{:?}", bytes);
        let size = bytes.len();
        let mut cursor = Cursor::new(bytes);

        producer.read_from(&mut cursor, Some(size)).unwrap();

        if !got_stream_info && producer.len() > threshold {
          match media_stream.stream_info() {
            Err(error) => log::error!("{}", error),
            Ok(()) => {
              got_stream_info = true;
              tx.send(Arc::new(Mutex::new(FormatContext::from(
                media_stream.format_context,
              ))))
              .unwrap();
            }
          }
        }
      }
    });

    let format_context = rx.recv().unwrap();

    SrtSource {
      _thread: thread,
      format_context,
      decoders: HashMap::new(),
    }
  }

  pub fn select_streams(&mut self, selection: Vec<StreamDescriptor>) -> Result<()> {
    self.decoders = HashMap::<usize, Decoder>::new();
    for selected_stream in &selection {
      let decoder = selected_stream
        .build_decoder(self.format_context.clone())
        .unwrap();
      self.decoders.insert(selected_stream.index, decoder);
    }

    Ok(())
  }

  pub fn next_frame(&mut self) -> Result<NextFrameResult> {
    let mut format_context = self.format_context.lock().unwrap();
    let res = format_context.next_packet();

    match res {
      Err(message) => {
        log::warn!("next_frame {:?}", message);
        if message == "Unable to read next packet" {
          // if self.thread.is_none() {
          //   return Ok(NextFrameResult::EndOfStream);
          // } else {
          return Ok(NextFrameResult::WaitMore);
          // }
        }

        if message == "End of data stream" {
          Ok(NextFrameResult::EndOfStream)
        } else {
          Err(Error::from(message))
        }
      }
      Ok(packet) => {
        let stream_index = packet.get_stream_index() as usize;
        log::debug!("Got Packet for index {}", stream_index);

        if let Some(decoder) = self.decoders.get_mut(&stream_index) {
          log::debug!("Decoder index {}", stream_index);
          // let mut is_sended = true;
          // while !is_sended {
          match decoder.decode(&packet) {
            Ok(Some(frame)) => {
              // println!("Got a frame !");
              // let time_base = Self::get_stream_time_base(stream_index as isize, &format_context);

              // if stream_index == self.get_first_stream_index() {
              //   self.position = Self::get_milliseconds_from_pts(frame.get_pts(), &time_base);

              //   // Check whether the end is not reached
              //   if let Some(segment_duration) = self.segment_duration {
              //     if self.position >= self.start_offset + segment_duration {
              //       return Ok(NextFrameResult::EndOfStream);
              //     }
              //   }
              // }

              // let start_pts = Self::get_pts_from_milliseconds(self.start_offset as i64, &time_base);

              // if frame.get_pts() < start_pts {
              //   log::trace!(
              //     "Need to decode more frames to reach the expected start PTS: {}/{}",
              //     frame.get_pts(),
              //     start_pts
              //   );

              // return Ok(NextFrameResult::WaitMore);
              //   return Ok(());
              // }

              Ok(NextFrameResult::Frame {
                stream_index,
                frame,
              })
            }
            Ok(None) => {
              Ok(NextFrameResult::WaitMore)
            }
            Err(message) => {
              log::error!("{:?}", message);
              if message == "Invalid data found when processing input" ||
                message == "Resource temporarily unavailable" {
                Ok(NextFrameResult::WaitMore)
              } else {
                Err(Error::from(message))                
              }
            }
          }
        } else {
          Ok(NextFrameResult::Nothing)
        }
      }
    }
  }
}
