pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Error::Io(error)
  }
}

impl From<String> for Error {
  fn from(error: String) -> Self {
    Error::Io(std::io::Error::new(std::io::ErrorKind::Other, error))
  }
}
