use super::*;

pub enum SourceStreamBuilder {
  File(FileSourceStream),
  Srt(SrtSourceStream),
}

impl SourceStreamBuilder {
  pub fn from_url(url: &str) -> Result<Self> {
    if url.starts_with("srt://") {
      let source_stream = SrtSourceStream::open(&url)?;
      Ok(Self::Srt(source_stream))
    } else {
      let source_stream = FileSourceStream::open(&url)?;
      Ok(Self::File(source_stream))
    }
  }

  pub fn get_source_stream(&mut self) -> &mut dyn SourceStream {
    match self {
      Self::File(source_stream) => source_stream,
      Self::Srt(source_stream) => source_stream,
    }
  }
}
