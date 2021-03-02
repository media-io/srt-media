use schemars::JsonSchema;
use stainless_ffmpeg::order::{Filter, ParameterValue};
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
// #[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct GenericFilter {
  pub name: String,
  pub label: Option<String>,
  pub parameters: HashMap<String, String>,
}

impl TryInto<Filter> for GenericFilter {
  type Error = crate::Error;

  fn try_into(self) -> Result<Filter, Self::Error> {
    let parameters = self
      .parameters
      .iter()
      .map(|(key, value)| (key.clone(), ParameterValue::String(value.clone())))
      .collect();

    let name = self.name.clone();
    let label = self.label;

    Ok(Filter {
      name,
      label,
      parameters,
      inputs: None,
      outputs: None,
    })
  }
}
