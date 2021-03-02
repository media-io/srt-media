#[cfg(all(feature = "python"))]
use dict_derive::{FromPyObject, IntoPyObject};
use schemars::JsonSchema;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct AudioFormat {
  pub sample_rates: Vec<usize>,
  pub channel_layouts: Vec<String>,
  pub sample_formats: Vec<String>,
}

impl Into<HashMap<String, String>> for &AudioFormat {
  fn into(self) -> HashMap<String, String> {
    let mut parameters = HashMap::new();

    if !self.sample_rates.is_empty() {
      let sample_rates = self
        .sample_rates
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("|");

      parameters.insert("sample_rates".to_string(), sample_rates);
    }
    if !self.channel_layouts.is_empty() {
      parameters.insert(
        "channel_layouts".to_string(),
        self.channel_layouts.join("|"),
      );
    }
    if !self.sample_formats.is_empty() {
      parameters.insert("sample_fmts".to_string(), self.sample_formats.join("|"));
    }

    parameters
  }
}
