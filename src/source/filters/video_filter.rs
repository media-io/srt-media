use super::*;
use stainless_ffmpeg::video_decoder::VideoDecoder;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VideoFilter {
  Crop(RegionOfInterest),
  Resize(VideoScaling),
  Format(VideoFormat),
  Generic(GenericFilter),
}

impl VideoFilter {
  pub fn as_generic_filter(&self, video_decoder: &VideoDecoder) -> Result<GenericFilter, String> {
    match self {
      VideoFilter::Crop(region_of_interest) => {
        let image_width = video_decoder.get_width() as u32;
        let image_height = video_decoder.get_height() as u32;
        let video_crop = region_of_interest.get_video_crop(image_width, image_height)?;

        Ok(GenericFilter {
          name: "crop".to_string(),
          label: Some("crop_filter".to_string()),
          parameters: (&video_crop).into(),
        })
      }
      VideoFilter::Resize(scaling) => Ok(GenericFilter {
        name: "scale".to_string(),
        label: Some("scale_filter".to_string()),
        parameters: scaling.into(),
      }),
      VideoFilter::Format(video_format) => Ok(GenericFilter {
        name: "format".to_string(),
        label: Some("format_filter".to_string()),
        parameters: video_format.into(),
      }),
      VideoFilter::Generic(generic_filter) => Ok(generic_filter.clone()),
    }
  }
}
