use anyhow::Result;
use iced::{
  Alignment, Element, Length, Pixels,
  alignment::Vertical,
  widget::{
    button, checkbox, column, combo_box, image, row, scrollable, slider, space, text, text_input,
  },
};
use yast_core::{
  layout::{component::Component, settings::SettingsValue},
  lua::settings::{SettingsFactoryEntryContent, SettingsFactoryValue},
};

use crate::{App, AppMessage};

pub fn component_editor<'a>(
  state: &'a App,
  p: &Component,
  full_path: Vec<usize>,
  mut path: Vec<usize>,
) -> Result<Element<'a, AppMessage>> {
  if path.len() > 0 {
    let popped = path.remove(0);
    let child = p
      .children
      .get(popped)
      .ok_or(anyhow::Error::msg("invalid path (no such child at index)"))?;
    component_editor(state, child, full_path, path)
  } else {
    let mut column_vec = Vec::new();

    column_vec.push(
      text("-- Tree --")
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .size(Pixels(30.))
        .into(),
    );

    let mut layout_part_attributes_row_vec = Vec::new();

    layout_part_attributes_row_vec.push(
      button("Delete Component")
        .on_press(AppMessage::DeleteComponent(full_path.clone()))
        .into(),
    );

    column_vec.push(row(layout_part_attributes_row_vec).padding(5.0).into());

    column_vec.push(
      row(vec![
        combo_box(
          &state.new_component_combo_box_state,
          "Components",
          state.new_component_combo_box_selected.as_ref(),
          |f| AppMessage::NewComponentComboBoxSelected(f),
        )
        .into(),
        space().width(Length::Fixed(5.0)).into(),
        button("Add Component")
          .on_press_maybe(
            state
              .new_component_combo_box_selected
              .as_ref()
              .map(|f| AppMessage::AddNewComponent(full_path.clone(), f.clone())),
          )
          .into(),
        space().width(Length::Fixed(5.0)).into(),
        button("Switch Component").into(),
      ])
      .padding(5.0)
      .into(),
    );

    column_vec.push(
      text("-- Parameters --")
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .size(Pixels(30.))
        .into(),
    );

    if let Some(comp_parameters) = state.layout.settings.get(&full_path) {
      let cpc = comp_parameters.clone();
      let lc = state.lua_context.lua.clone();
      let rc = state.repository.clone();
      let fpc = full_path.clone();
      let lua_setting_function =
        state
          .lua_context
          .lua
          .create_function(move |_, name: String| {
            if let Some(value) = cpc.get(&name) {
              Ok(value.inner(&lc, &rc, fpc.clone(), name))
            } else {
              Err(mlua::Error::external(anyhow::Error::msg(format!(
                "couldn't find setting {} in component",
                name
              ))))
            }
          })?;

      for param in &p.parameters.0 {
        let lsfc = lua_setting_function.clone();
        let visible = match &param.show_if {
          None => true,
          Some(f) => f.call(lsfc)?,
        };

        if visible {
          match &param.content {
            SettingsFactoryEntryContent::Header(title) => {
              column_vec.push(
                text(format!("- {} -", title))
                  .width(Length::Fill)
                  .align_x(Alignment::Center)
                  .into(),
              );
            }
            SettingsFactoryEntryContent::Value(name, factory_value) => {
              if let Some(value) = comp_parameters.get(name) {
                match value {
                  SettingsValue::Boolean(value) => {
                    let moved_name = name.clone();
                    let moved_full_path = full_path.clone();

                    column_vec.push(
                      row(vec![
                        text(name.clone()).into(),
                        space().width(Length::Fixed(5.0)).into(),
                        checkbox(*value)
                          .on_toggle(move |new| {
                            AppMessage::ModifyParameterBoolean(
                              moved_full_path.clone(),
                              moved_name.clone(),
                              new,
                            )
                          })
                          .into(),
                      ])
                      .into(),
                    );
                  }
                  SettingsValue::String(value) => {
                    let moved_name = name.clone();
                    let moved_full_path = full_path.clone();
                    let moved_current = value.clone();

                    column_vec.push(
                      row(vec![
                        text(name.clone()).into(),
                        space().width(Length::Fixed(5.0)).into(),
                        text_input(&name, &moved_current)
                          .on_input(move |new| {
                            AppMessage::ModifyParameterString(
                              moved_full_path.clone(),
                              moved_name.clone(),
                              new,
                            )
                          })
                          .into(),
                      ])
                      .align_y(Vertical::Center)
                      .into(),
                    );
                  }
                  SettingsValue::Options(value) => {
                    if let SettingsFactoryValue::Options(_, _) = factory_value {
                      if let Some(st) = state.parameter_options_combo_box_states.get(name) {
                        let moved_name = name.clone();
                        let moved_full_path = full_path.clone();

                        column_vec.push(
                          row(vec![
                            text(format!("{}", name)).into(),
                            space().width(Length::Fixed(5.0)).into(),
                            combo_box(st, "", Some(value), move |s| {
                              AppMessage::ModifyParameterOptions(
                                moved_full_path.clone(),
                                moved_name.clone(),
                                s,
                              )
                            })
                            .into(),
                          ])
                          .align_y(Vertical::Center)
                          .into(),
                        );
                      }
                    }
                  }
                  SettingsValue::Number(value) => {
                    let moved_name = name.clone();
                    let moved_full_path = full_path.clone();

                    column_vec.push(
                      row(vec![
                        text(format!("{}", name)).into(),
                        space().width(Length::Fixed(5.0)).into(),
                        text_input(&name, &format!("{}", value))
                          .on_input(move |new| {
                            AppMessage::ModifyParameterNumber(
                              moved_full_path.clone(),
                              moved_name.clone(),
                              new,
                            )
                          })
                          .into(),
                      ])
                      .align_y(Vertical::Center)
                      .into(),
                    );
                  }
                  SettingsValue::NumberRange(value) => {
                    if let SettingsFactoryValue::NumberRange(min, max, step, _) = factory_value {
                      let moved_name = name.clone();
                      let moved_full_path = full_path.clone();

                      column_vec.push(
                        row(vec![
                          text(format!("{}", name)).into(),
                          space().width(Length::Fixed(5.0)).into(),
                          slider(*min..=*max, *value, move |new| {
                            AppMessage::ModifyParameterNumberRange(
                              moved_full_path.clone(),
                              moved_name.clone(),
                              new,
                            )
                          })
                          .step(*step)
                          .into(),
                          space().width(Length::Fixed(5.0)).into(),
                          text(format!("{}", value)).width(Length::Fixed(50.0)).into(),
                        ])
                        .align_y(Vertical::Center)
                        .into(),
                      );
                    }
                  }
                  SettingsValue::Color(value) => {
                    let moved_name_0 = name.clone();
                    let moved_full_path_0 = full_path.clone();
                    let moved_name_1 = name.clone();
                    let moved_full_path_1 = full_path.clone();
                    let moved_name_2 = name.clone();
                    let moved_full_path_2 = full_path.clone();
                    let moved_name_3 = name.clone();
                    let moved_full_path_3 = full_path.clone();

                    column_vec.push(
                      row(vec![
                        text(format!("{}", name)).into(),
                        text_input(&name, &format!("{}", value[0] * 255.))
                          .on_input(move |new| {
                            AppMessage::ModifyParameterColor(
                              moved_full_path_0.clone(),
                              moved_name_0.clone(),
                              0,
                              new,
                            )
                          })
                          .into(),
                        text_input(&name, &format!("{}", value[1] * 255.))
                          .on_input(move |new| {
                            AppMessage::ModifyParameterColor(
                              moved_full_path_1.clone(),
                              moved_name_1.clone(),
                              1,
                              new,
                            )
                          })
                          .into(),
                        text_input(&name, &format!("{}", value[2] * 255.))
                          .on_input(move |new| {
                            AppMessage::ModifyParameterColor(
                              moved_full_path_2.clone(),
                              moved_name_2.clone(),
                              2,
                              new,
                            )
                          })
                          .into(),
                        text_input(&name, &format!("{}", value[3] * 255.))
                          .on_input(move |new| {
                            AppMessage::ModifyParameterColor(
                              moved_full_path_3.clone(),
                              moved_name_3.clone(),
                              3,
                              new,
                            )
                          })
                          .into(),
                        text(format!(
                          "#{:02x}{:02x}{:02x}{:02x}",
                          (value[0] * 255.) as u32,
                          (value[1] * 255.) as u32,
                          (value[2] * 255.) as u32,
                          (value[3] * 255.) as u32
                        ))
                        .into(),
                      ])
                      .spacing(5.0)
                      .align_y(Vertical::Center)
                      .into(),
                    );
                  }
                  SettingsValue::Image(_) => {
                    let mut row_vec = vec![
                      text(format!("{}", name)).into(),
                      space().width(Length::Fixed(5.0)).into(),
                      button("Open Image")
                        .on_press(AppMessage::ModifyParameterImageOpen(
                          full_path.clone(),
                          name.clone(),
                        ))
                        .into(),
                    ];

                    if let Some(h) = state
                      .repository
                      .layout_images
                      .get(&(full_path.clone(), name.clone()))
                      .ok_or(anyhow::Error::msg("couldn't find image in repository"))?
                    {
                      row_vec.push(
                        image(h)
                          .width(Length::Fixed(100.))
                          .height(Length::Fixed(100.))
                          .into(),
                      );
                    }

                    column_vec.push(row(row_vec).align_y(Vertical::Center).into());
                  }
                }
              }
            }
          }
        }
      }
    }

    let column: Element<'a, AppMessage> =
      column(column_vec).padding(10.0).width(Length::Fill).into();

    Ok(
      scrollable(column)
        .width(Length::FillPortion(3))
        .height(Length::Fill)
        .into(),
    )
  }
}
