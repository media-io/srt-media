use schemars::JsonSchema;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct VideoScaling {
  pub width: Option<u32>,
  pub height: Option<u32>,
}

impl Into<HashMap<String, String>> for &VideoScaling {
  fn into(self) -> HashMap<String, String> {
    let width = self.width.map_or((-1).to_string(), |w| w.to_string());
    let height = self.height.map_or((-1).to_string(), |h| h.to_string());

    [("width", width), ("height", height)]
      .iter()
      .map(|(key, value)| (key.to_string(), value.clone()))
      .collect()
  }
}

#[test]
pub fn test_video_scaling_get_filter_parameters() {
  let scaling = VideoScaling {
    width: None,
    height: None,
  };
  let parameters: HashMap<String, String> = (&scaling).into();
  assert_eq!(&(-1).to_string(), parameters.get("width").unwrap());
  assert_eq!(&(-1).to_string(), parameters.get("height").unwrap());

  let scaling = VideoScaling {
    width: Some(1234),
    height: None,
  };
  let parameters: HashMap<String, String> = (&scaling).into();
  assert_eq!(&1234.to_string(), parameters.get("width").unwrap());
  assert_eq!(&(-1).to_string(), parameters.get("height").unwrap());

  let scaling = VideoScaling {
    width: None,
    height: Some(1234),
  };
  let parameters: HashMap<String, String> = (&scaling).into();
  assert_eq!(&(-1).to_string(), parameters.get("width").unwrap());
  assert_eq!(&1234.to_string(), parameters.get("height").unwrap());

  let scaling = VideoScaling {
    width: Some(1234),
    height: Some(5678),
  };
  let parameters: HashMap<String, String> = (&scaling).into();
  assert_eq!(&1234.to_string(), parameters.get("width").unwrap());
  assert_eq!(&5678.to_string(), parameters.get("height").unwrap());
}
