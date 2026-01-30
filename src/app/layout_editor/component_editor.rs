use anyhow::Result;
use iced::{
  Alignment, Element, Length,
  widget::{column, text},
};

use crate::{app::AppMessage, layout::LayoutPart};

pub fn component_editor<'a>(
  p: &Box<dyn LayoutPart>,
  mut path: Vec<usize>,
) -> Result<Element<'a, AppMessage>> {
  if path.len() > 0 {
    let popped = path.remove(0);
    let child = p
      .get_children()
      .ok_or(anyhow::Error::msg("invalid path (no children)"))?
      .get(popped)
      .ok_or(anyhow::Error::msg("invalid path (no such child at index)"))?;
    component_editor(child, path)
  } else {
    let mut column_vec = Vec::new();

    column_vec.push(
      text(format!("Editing: {} - {}", p.get_name(), p.get_author()))
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into(),
    );

    Ok(
      column(column_vec)
        .padding(2.0)
        .width(Length::FillPortion(3))
        .into(),
    )
  }
}
