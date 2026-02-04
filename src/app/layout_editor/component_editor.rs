use anyhow::Result;
use iced::{
  Alignment, Background, Color, Element, Length,
  alignment::Vertical,
  widget::{button, checkbox, column, combo_box, image, row, slider, space, text, text_input},
};
use iced_aw::color_picker;

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
      match &param.value {
        LuaComponentSettingValue::Boolean { value, default: _ } => {
          let moved_name = name.clone();
          let moved_full_path = full_path.clone();

          column_vec.push(
            row(vec![
              text(name.clone()).into(),
              space().width(Length::Fixed(5.0)).into(),
              checkbox(*value)
                .on_toggle(move |new| {
                  AppMessage::LayoutEditor(LayoutEditorMessage::ModifyParameterBoolean(
                    moved_full_path.clone(),
                    moved_name.clone(),
                    new,
                  ))
                })
                .into(),
            ])
            .into(),
          );
        }
        LuaComponentSettingValue::String { value, default: _ } => {
          let moved_name = name.clone();
          let moved_full_path = full_path.clone();
          let moved_current = value.clone();

          column_vec.push(
            row(vec![
              text(name.clone()).into(),
              space().width(Length::Fixed(5.0)).into(),
              text_input(&name, &moved_current)
                .on_input(move |new| {
                  AppMessage::LayoutEditor(LayoutEditorMessage::ModifyParameterString(
                    moved_full_path.clone(),
                    moved_name.clone(),
                    new,
                  ))
                })
                .into(),
            ])
            .align_y(Vertical::Center)
            .into(),
          );
        }
        LuaComponentSettingValue::Options {
          value,
          default: _,
          options: _,
        } => {
          if let Some(st) = state.parameter_options_combo_box_states.get(&name) {
            let moved_name = name.clone();
            let moved_full_path = full_path.clone();

            column_vec.push(
              row(vec![
                text(format!("{}", name)).into(),
                space().width(Length::Fixed(5.0)).into(),
                combo_box(st, "", Some(value), move |s| {
                  AppMessage::LayoutEditor(LayoutEditorMessage::ModifyParameterOptions(
                    moved_full_path.clone(),
                    moved_name.clone(),
                    s,
                  ))
                })
                .into(),
              ])
              .align_y(Vertical::Center)
              .into(),
            );
          }
        }
        LuaComponentSettingValue::Number { value, default: _ } => {
          let moved_name = name.clone();
          let moved_full_path = full_path.clone();

          column_vec.push(
            row(vec![
              text(format!("{}", name)).into(),
              space().width(Length::Fixed(5.0)).into(),
              text_input(&name, &format!("{}", value))
                .on_input(move |new| {
                  AppMessage::LayoutEditor(LayoutEditorMessage::ModifyParameterNumber(
                    moved_full_path.clone(),
                    moved_name.clone(),
                    new,
                  ))
                })
                .into(),
            ])
            .align_y(Vertical::Center)
            .into(),
          );
        }
        LuaComponentSettingValue::NumberRange {
          value,
          default: _,
          min,
          max,
          step,
        } => {
          let moved_name = name.clone();
          let moved_full_path = full_path.clone();

          column_vec.push(
            row(vec![
              text(format!("{}", name)).into(),
              space().width(Length::Fixed(5.0)).into(),
              slider(*min..=*max, *value, move |new| {
                AppMessage::LayoutEditor(LayoutEditorMessage::ModifyParameterNumberRange(
                  moved_full_path.clone(),
                  moved_name.clone(),
                  new,
                ))
              })
              .step(*step)
              .into(),
              space().width(Length::Fixed(5.0)).into(),
              text(format!("{}", value)).into(),
            ])
            .align_y(Vertical::Center)
            .into(),
          );
        }
        LuaComponentSettingValue::Color { value, default: _ } => {
          if let Some(opened) = state.parameter_options_color_picker_opened.get(&name) {
            let moved_name = name.clone();
            let moved_full_path = full_path.clone();
            let color_color = Color::from_rgba(value[0], value[1], value[2], value[3]);

            column_vec.push(
              row(vec![
                text(format!("{}", name)).into(),
                space().width(Length::Fixed(5.0)).into(),
                color_picker(
                  *opened,
                  color_color,
                  button(space())
                    .style(move |_, _| button::Style {
                      background: Some(Background::Color(color_color)),
                      ..Default::default()
                    })
                    .on_press(AppMessage::LayoutEditor(
                      LayoutEditorMessage::ModifyParameterColorOpen(name.clone()),
                    )),
                  AppMessage::LayoutEditor(LayoutEditorMessage::ModifyParameterColorCancel(
                    name.clone(),
                  )),
                  move |n| {
                    AppMessage::LayoutEditor(LayoutEditorMessage::ModifyParameterColorSubmit(
                      moved_full_path.clone(),
                      moved_name.clone(),
                      n,
                    ))
                  },
                )
                .into(),
                space().width(Length::Fixed(5.0)).into(),
                text(format!(
                  "#{:02x}{:02x}{:02x}{:02x}",
                  (value[0] * 255.) as u32,
                  (value[1] * 255.) as u32,
                  (value[2] * 255.) as u32,
                  (value[3] * 255.) as u32
                ))
                .into(),
              ])
              .align_y(Vertical::Center)
              .into(),
            );
          }
        }
        LuaComponentSettingValue::Image { bytes: _ } => {
          let mut row_vec = vec![
            text(format!("{}", name)).into(),
            space().width(Length::Fixed(5.0)).into(),
            button("Open Image")
              .on_press(AppMessage::LayoutEditor(
                LayoutEditorMessage::ModifyParameterImageOpen(full_path.clone(), name.clone()),
              ))
              .into(),
          ];

          if let Some(b) = state.parameter_options_image_handles.get(&name) {
            row_vec.push(
              image(b)
                .width(Length::Fixed(100.))
                .height(Length::Fixed(100.))
                .into(),
            );
          }

          column_vec.push(row(row_vec).align_y(Vertical::Center).into());
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
