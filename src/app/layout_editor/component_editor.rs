use anyhow::Result;
use iced::{
  Alignment, Element, Length, Pixels,
  widget::{button, column, combo_box, row, space, text},
};

use crate::{
  app::{
    AppMessage,
    layout_editor::{LayoutEditor, LayoutEditorMessage},
  },
  layout::LayoutPart,
};

pub fn component_editor<'a>(
  state: &'a LayoutEditor,
  p: &Box<dyn LayoutPart>,
  full_path: Vec<usize>,
  mut path: Vec<usize>,
) -> Result<Element<'a, AppMessage>> {
  if path.len() > 0 {
    let popped = path.remove(0);
    let child = p
      .get_children()
      .ok_or(anyhow::Error::msg("invalid path (no children)"))?
      .get(popped)
      .ok_or(anyhow::Error::msg("invalid path (no such child at index)"))?;
    component_editor(state, child, full_path, path)
  } else {
    let children = p.get_children();

    let mut column_vec = Vec::new();

    column_vec.push(
      text(format!("Editing: {} - {}", p.get_name(), p.get_author()))
        .size(Pixels(20.))
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into(),
    );

    column_vec.push(
      text("-- Layout Part Attributes --")
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into(),
    );

    let mut layout_part_attributes_row_vec = Vec::new();

    if full_path.len() > 0 {
      layout_part_attributes_row_vec.push(
        button("Delete Part")
          .on_press(AppMessage::LayoutEditor(
            LayoutEditorMessage::DeleteComponent(full_path.clone()),
          ))
          .into(),
      );

      layout_part_attributes_row_vec.push(button("Move Up").into());

      layout_part_attributes_row_vec.push(button("Move Down").into());
    }

    column_vec.push(row(layout_part_attributes_row_vec).padding(5.0).into());

    if let Some(children) = children {
      column_vec.push(
        text("-- Container Attributes --")
          .width(Length::Fill)
          .align_x(Alignment::Center)
          .into(),
      );

      column_vec.push(
        row(vec![
          combo_box(
            &state.new_component_combo_box_state,
            "Parts",
            state.new_component_combo_box_selected.as_ref(),
            |f| AppMessage::LayoutEditor(LayoutEditorMessage::NewComponentComboBoxSelected(f)),
          )
          .into(),
          button("Add Part")
            .on_press_maybe(state.new_component_combo_box_selected.as_ref().map(|f| {
              AppMessage::LayoutEditor(LayoutEditorMessage::AddNewComponent(full_path, f.clone()))
            }))
            .into(),
        ])
        .padding(5.0)
        .into(),
      );
    }

    Ok(
      column(column_vec)
        .padding(10.0)
        .width(Length::FillPortion(3))
        .into(),
    )
  }
}
