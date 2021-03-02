use crate::source::filters::{GenericFilter, VideoFilter};
use crate::Result;
use stainless_ffmpeg::{filter_graph::FilterGraph, order::Filter, video_decoder::VideoDecoder};
use std::convert::TryInto;

pub fn build_video_filter_graph(
  video_filters: &[VideoFilter],
  video_decoder: &VideoDecoder,
) -> Result<Option<FilterGraph>> {
  let mut graph = FilterGraph::new()?;

  let mut filters: Vec<_> = video_filters
    .iter()
    .map(|video_filter| {
      let generic_filter: GenericFilter = video_filter.as_generic_filter(video_decoder).unwrap();
      let filter: Filter = generic_filter.try_into().unwrap();

      graph.add_filter(&filter).unwrap()
    })
    .collect();

  if filters.is_empty() {
    return Ok(None);
  }

  graph.add_input_from_video_decoder("video_input", video_decoder)?;

  graph.add_video_output("video_output")?;

  // connect filters, except the first one
  let mut filter = filters.remove(0);
  log::trace!(
    "Connect video graph input to filter {}...",
    filter.get_label()
  );
  graph.connect_input("video_input", 0, &filter, 0)?;

  while !filters.is_empty() {
    let next_filter = filters.remove(0);
    log::trace!(
      "Connect filter {} to filter {}...",
      filter.get_label(),
      next_filter.get_label()
    );

    graph.connect(&filter, 0, &next_filter, 0)?;

    filter = next_filter;
  }

  log::trace!(
    "Connect filter {} to video graph output...",
    filter.get_label()
  );
  graph.connect_output(&filter, 0, "video_output", 0)?;

  graph
    .validate()
    .map_err(|error| format!("Video filter graph validation failed: {}", error))?;
  Ok(Some(graph))
}
