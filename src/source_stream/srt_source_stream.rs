use super::SourceStream;
use crate::Result;
use bytes::Bytes;
use futures_util::TryStreamExt;
use srt_tokio::{SrtSocket, SrtSocketBuilder};
use std::{cell::RefCell, rc::Rc, time::Instant};
use tokio::runtime::Runtime;

pub struct SrtSourceStream {
  socket: Rc<RefCell<SrtSocket>>,
  runtime: Runtime,
}

impl SrtSourceStream {
  pub fn open(url: &str) -> Result<Self> {
    let runtime = Runtime::new().unwrap();

    let socket = runtime.block_on(async {
      if url.starts_with("srt://:") {
        let port = url.replace("srt://:", "").parse::<u16>().unwrap();
        SrtSocketBuilder::new_listen()
          .local_port(port)
          .connect()
          .await
          .unwrap()
      } else {
        let url = url.replace("srt://", "");

        SrtSocketBuilder::new_connect(url).connect().await.unwrap()
      }
    });

    let socket = Rc::new(RefCell::new(socket));

    log::info!("SRT connected");

    Ok(SrtSourceStream { socket, runtime })
  }
}

impl SourceStream for SrtSourceStream {
  fn receive(&mut self) -> Option<(Instant, Bytes)> {
    let socket = self.socket.clone();
    self
      .runtime
      .block_on(async { socket.borrow_mut().try_next().await.unwrap() })
  }
}
