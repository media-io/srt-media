use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "python", derive(FromPyObject, IntoPyObject))]
pub struct VideoCrop {
  pub top: u32,
  pub left: u32,
  pub width: u32,
  pub height: u32,
}

impl Into<HashMap<String, String>> for &VideoCrop {
  fn into(self) -> HashMap<String, String> {
    [
      ("out_w", self.width.to_string()),
      ("out_h", self.height.to_string()),
      ("x", self.left.to_string()),
      ("y", self.top.to_string()),
    ]
    .iter()
    .cloned()
    .map(|(key, value)| (key.to_string(), value))
    .collect()
  }
}

#[test]
pub fn test_video_crop_get_filter_parameters() {
  let crop = VideoCrop {
    top: 147,
    left: 258,
    width: 123,
    height: 456,
  };

  let parameters: HashMap<String, String> = (&crop).into();

  assert_eq!(&147.to_string(), parameters.get("y").unwrap());
  assert_eq!(&258.to_string(), parameters.get("x").unwrap());
  assert_eq!(&123.to_string(), parameters.get("out_w").unwrap());
  assert_eq!(&456.to_string(), parameters.get("out_h").unwrap());
}
