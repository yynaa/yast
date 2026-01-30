use anyhow::Result;
use iced::{
  Alignment, Element, Length, Pixels,
  widget::{button, checkbox, column, combo_box, row, space, text},
};

use crate::{
  app::{
    AppMessage,
    layout_editor::{LayoutEditor, LayoutEditorMessage},
  },
  layout::LayoutPart,
  lua::settings::LuaComponentSettingValue,
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
      text("-- Tree --")
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into(),
    );

    let mut layout_part_attributes_row_vec = Vec::new();

    layout_part_attributes_row_vec.push(
      button("Delete Component")
        .on_press(AppMessage::LayoutEditor(
          LayoutEditorMessage::DeleteComponent(full_path.clone()),
        ))
        .into(),
    );

    if full_path.len() > 0 {
      layout_part_attributes_row_vec.push(space().width(Length::Fixed(5.0)).into());

      layout_part_attributes_row_vec.push(
        button("Move Up")
          .on_press(AppMessage::LayoutEditor(
            LayoutEditorMessage::MoveComponentUp(full_path.clone()),
          ))
          .into(),
      );
      layout_part_attributes_row_vec.push(
        button("Move Down")
          .on_press(AppMessage::LayoutEditor(
            LayoutEditorMessage::MoveComponentDown(full_path.clone()),
          ))
          .into(),
      );

      layout_part_attributes_row_vec.push(space().width(Length::Fixed(5.0)).into());

      layout_part_attributes_row_vec.push(
        button("Enter Above")
          .on_press(AppMessage::LayoutEditor(
            LayoutEditorMessage::EnterAboveComponent(full_path.clone()),
          ))
          .into(),
      );
      layout_part_attributes_row_vec.push(
        button("Exit Parent")
          .on_press(AppMessage::LayoutEditor(
            LayoutEditorMessage::ExitParentComponent(full_path.clone()),
          ))
          .into(),
      );
    }

    column_vec.push(row(layout_part_attributes_row_vec).padding(5.0).into());

    if let Some(_) = children {
      column_vec.push(
        row(vec![
          combo_box(
            &state.new_component_combo_box_state,
            "Components",
            state.new_component_combo_box_selected.as_ref(),
            |f| AppMessage::LayoutEditor(LayoutEditorMessage::NewComponentComboBoxSelected(f)),
          )
          .into(),
          space().width(Length::Fixed(5.0)).into(),
          button("Add Component")
            .on_press_maybe(state.new_component_combo_box_selected.as_ref().map(|f| {
              AppMessage::LayoutEditor(LayoutEditorMessage::AddNewComponent(
                full_path.clone(),
                f.clone(),
              ))
            }))
            .into(),
          space().width(Length::Fixed(5.0)).into(),
          button("Switch Component").into(),
        ])
        .padding(5.0)
        .into(),
      );
    }

    column_vec.push(
      text("-- Parameters --")
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into(),
    );

    for param in &p.get_parameters().values {
      let name = param.name.clone();
      match param.value {
        LuaComponentSettingValue::Boolean { value, default: _ } => {
          let moved_name = name.clone();
          let moved_full_path = full_path.clone();

          column_vec.push(
            checkbox(value)
              .label(name)
              .on_toggle(move |new| {
                AppMessage::LayoutEditor(LayoutEditorMessage::ModifyParameterBoolean(
                  moved_full_path.clone(),
                  moved_name.clone(),
                  new,
                ))
              })
              .into(),
          );
        }
      }
    }

    Ok(
      column(column_vec)
        .padding(10.0)
        .width(Length::FillPortion(3))
        .into(),
    )
  }
}
