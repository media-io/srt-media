use super::*;
use std::convert::TryInto;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AudioFilter {
  Format(AudioFormat),
  Generic(GenericFilter),
}

impl TryInto<GenericFilter> for &AudioFilter {
  type Error = String;

  fn try_into(self) -> Result<GenericFilter, Self::Error> {
    let filter = match self {
      AudioFilter::Format(audio_format) => GenericFilter {
        name: "aformat".to_string(),
        label: Some("aformat_filter".to_string()),
        parameters: audio_format.into(),
      },
      AudioFilter::Generic(generic_filter) => generic_filter.clone(),
    };

    Ok(filter)
  }
}
