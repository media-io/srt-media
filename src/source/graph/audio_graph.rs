use crate::source::filters::{AudioFilter, GenericFilter};
use crate::Result;
use stainless_ffmpeg::{audio_decoder::AudioDecoder, filter_graph::FilterGraph, order::Filter};
use std::convert::TryInto;

pub fn build_audio_filter_graph(
  audio_filters: &[AudioFilter],
  audio_decoder: &AudioDecoder,
) -> Result<Option<FilterGraph>> {
  let mut graph = FilterGraph::new()?;

  let mut filters: Vec<_> = audio_filters
    .iter()
    .map(|audio_filter| {
      let generic_filter: GenericFilter = audio_filter.try_into().unwrap();
      let filter: Filter = generic_filter.try_into().unwrap();
      let filter = graph.add_filter(&filter).unwrap();
      filter
    })
    .collect();

  if filters.is_empty() {
    return Ok(None);
  }

  graph.add_input_from_audio_decoder("audio_input", audio_decoder)?;

  graph.add_audio_output("audio_output")?;

  // connect filters, except the first one
  let mut filter = filters.remove(0);
  log::trace!(
    "Connect audio graph input to filter {}...",
    filter.get_label()
  );
  graph.connect_input("audio_input", 0, &filter, 0)?;

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
    "Connect filter {} to audio graph output...",
    filter.get_label()
  );
  graph.connect_output(&filter, 0, "audio_output", 0)?;

  graph
    .validate()
    .map_err(|error| format!("Audio filter graph validation failed: {}", error))?;

  Ok(Some(graph))
}
