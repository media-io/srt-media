use crate::Result;
use bytes::Bytes;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub struct FileConnection {
  file: File,
}

impl FileConnection {
  pub fn open_connection(url: &str) -> Result<Self> {
    let file = File::open(url)?;
    Ok(FileConnection { file })
  }

  pub fn receive(&mut self) -> Option<(Instant, Bytes)> {
    let mut data = vec![0; 1316];

    self.file.read_exact(&mut data).unwrap();

    Some((Instant::now(), Bytes::from(data)))
  }
}
