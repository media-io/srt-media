use schemars::JsonSchema;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct VideoFormat {
  pub pixel_formats: String,
}

impl Into<HashMap<String, String>> for &VideoFormat {
  fn into(self) -> HashMap<String, String> {
    let mut parameters = HashMap::<String, String>::new();
    parameters.insert("pix_fmts".to_string(), self.pixel_formats.clone());
    parameters
  }
}
