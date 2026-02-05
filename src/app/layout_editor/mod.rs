use std::{collections::HashMap, fs};

use iced::{
  Color, Element, Length, Task,
  advanced::image,
  widget::{button, column, combo_box, row, text, text_input},
  window,
};

use crate::{
  app::{
    AppContext, AppMessage, Window,
    layout_editor::{
      component_editor::component_editor,
      tree::{build_tree_from_layout_part, get_mut_component_at_path},
    },
  },
  layout::component::Component,
  lua::settings::LuaComponentSettingValue,
};

mod component_editor;
mod tree;

pub struct LayoutEditor {
  pub opened_component: Vec<usize>,

  pub new_component_combo_box_state: combo_box::State<String>,
  pub new_component_combo_box_selected: Option<String>,

  pub parameter_options_combo_box_states: HashMap<String, combo_box::State<String>>,
  pub parameter_options_color_picker_opened: HashMap<String, bool>,
}

#[derive(Clone, Debug)]
pub enum LayoutEditorMessage {
  LayoutNameChanged(String),
  LayoutAuthorChanged(String),

  OpenComponent(Vec<usize>),

  NewComponentComboBoxSelected(String),
  AddNewComponent(Vec<usize>, String),

  DeleteComponent(Vec<usize>),

  MoveComponentUp(Vec<usize>),
  MoveComponentDown(Vec<usize>),
  EnterAboveComponent(Vec<usize>),
  ExitParentComponent(Vec<usize>),

  ModifyParameterBoolean(Vec<usize>, String, bool),
  ModifyParameterString(Vec<usize>, String, String),
  ModifyParameterOptions(Vec<usize>, String, String),
  ModifyParameterNumber(Vec<usize>, String, String),
  ModifyParameterNumberRange(Vec<usize>, String, f64),
  ModifyParameterColorOpen(String),
  ModifyParameterColorCancel(String),
  ModifyParameterColorSubmit(Vec<usize>, String, Color),
  ModifyParameterImageOpen(Vec<usize>, String),
  ModifyParameterImageSubmit(Vec<usize>, String, Vec<u8>),
}

impl LayoutEditor {
  pub fn open_window() -> Task<window::Id> {
    window::open(window::Settings {
      ..Default::default()
    })
    .1
  }

  pub fn new(context: &AppContext) -> Self {
    let mut new_component_options = Vec::new();

    new_component_options.append(
      &mut context
        .components
        .keys()
        .map(|s| s.clone())
        .collect::<Vec<String>>(),
    );

    Self {
      opened_component: Vec::new(),
      new_component_combo_box_state: combo_box::State::new(new_component_options),
      new_component_combo_box_selected: None,
      parameter_options_combo_box_states: HashMap::new(),
      parameter_options_color_picker_opened: HashMap::new(),
    }
  }
}

impl Window for LayoutEditor {
  fn title(&self) -> String {
    String::from("YAST Layout Editor")
  }

  fn update(&mut self, context: &mut AppContext, message: AppMessage) -> Task<AppMessage> {
    match message {
      AppMessage::LayoutEditor(inner) => match inner {
        LayoutEditorMessage::LayoutNameChanged(n) => {
          context.layout.name = n;
          Task::none()
        }
        LayoutEditorMessage::LayoutAuthorChanged(n) => {
          context.layout.author = n;
          Task::none()
        }
        LayoutEditorMessage::OpenComponent(n) => {
          if let Some(lcontent) = &mut context.layout.content {
            self.opened_component = n.clone();
            self.new_component_combo_box_selected = None;
            self.parameter_options_combo_box_states.clear();
            self.parameter_options_color_picker_opened.clear();

            let comp = get_mut_component_at_path(lcontent, n).unwrap();
            for p in &comp.get_parameters().values {
              match &p.value {
                LuaComponentSettingValue::Options {
                  value: _,
                  default: _,
                  options,
                } => {
                  self
                    .parameter_options_combo_box_states
                    .insert(p.name.clone(), combo_box::State::new(options.clone()));
                }
                LuaComponentSettingValue::Color {
                  value: _,
                  default: _,
                } => {
                  self
                    .parameter_options_color_picker_opened
                    .insert(p.name.clone(), false);
                }
                _ => {}
              }
            }
          }

          Task::none()
        }
        LayoutEditorMessage::NewComponentComboBoxSelected(n) => {
          self.new_component_combo_box_selected = Some(n);
          Task::none()
        }
        LayoutEditorMessage::AddNewComponent(path, name) => {
          if let Some(lcontent) = &mut context.layout.content {
            let parent = get_mut_component_at_path(lcontent, path).unwrap();
            let parent_children = parent.get_children_mut().unwrap();
            parent_children.push(Box::new(
              Component::from_str(
                context.components.get(&name).unwrap().clone(),
                &context.lua_context.lua,
              )
              .unwrap(),
            ));
            Task::none()
          } else {
            context.layout.content = Some(Box::new(
              Component::from_str(
                context.components.get(&name).unwrap().clone(),
                &context.lua_context.lua,
              )
              .unwrap(),
            ));
            Task::done(AppMessage::LayoutEditor(
              LayoutEditorMessage::OpenComponent(vec![]),
            ))
          }
        }
        LayoutEditorMessage::DeleteComponent(mut path) => {
          if let Some(lcontent) = &mut context.layout.content {
            if path.len() > 0 {
              let last_path_element = path.pop().unwrap();
              let parent = get_mut_component_at_path(lcontent, path).unwrap();
              parent.get_children_mut().unwrap().remove(last_path_element);
              self.opened_component.pop();
              Task::none()
            } else {
              context.layout.content = None;
              Task::none()
            }
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::MoveComponentUp(mut path) => {
          if let Some(lcontent) = &mut context.layout.content {
            let last_path_element = path.pop().unwrap();
            if last_path_element > 0 {
              let parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
              let parent_children = parent.get_children_mut().unwrap();
              let to_move = parent_children.remove(last_path_element);
              parent_children.insert(last_path_element - 1, to_move);
              self.opened_component = path.clone();
              self.opened_component.push(last_path_element - 1);
            }

            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::MoveComponentDown(mut path) => {
          if let Some(lcontent) = &mut context.layout.content {
            let last_path_element = path.pop().unwrap();
            let parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
            let parent_children = parent.get_children_mut().unwrap();
            if last_path_element < parent_children.len() - 1 {
              let to_move = parent_children.remove(last_path_element);
              parent_children.insert(last_path_element + 1, to_move);
              self.opened_component = path.clone();
              self.opened_component.push(last_path_element + 1);
            }

            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::EnterAboveComponent(mut path) => {
          if let Some(lcontent) = &mut context.layout.content {
            let last_path_element = path.pop().unwrap();
            if last_path_element > 0 {
              let parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
              let parent_children = parent.get_children_mut().unwrap();
              let to_move = parent_children.remove(last_path_element);
              let new_parent = parent_children.get_mut(last_path_element - 1).unwrap();
              let new_parent_children = new_parent.get_children_mut().unwrap();
              new_parent_children.push(to_move);
              self.opened_component = path.clone();
              self.opened_component.push(last_path_element - 1);
              self.opened_component.push(new_parent_children.len() - 1);
            }

            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::ExitParentComponent(mut path) => {
          if let Some(lcontent) = &mut context.layout.content {
            if path.len() > 1 {
              let last_path_element = path.pop().unwrap();
              let second_last_path_element = path.pop().unwrap();
              let parent_parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
              let parent_parent_children = parent_parent.get_children_mut().unwrap();
              let myself = parent_parent_children
                .get_mut(second_last_path_element)
                .unwrap()
                .get_children_mut()
                .unwrap()
                .remove(last_path_element);
              parent_parent_children.insert(second_last_path_element, myself.clone());
              self.opened_component = path.clone();
              self.opened_component.push(second_last_path_element);
            }

            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::ModifyParameterBoolean(path, param, value) => {
          if let Some(lcontent) = &mut context.layout.content {
            let component = get_mut_component_at_path(lcontent, path).unwrap();
            let parameters = component.get_parameters_mut();
            let to_edit = parameters
              .values
              .iter_mut()
              .find(|v| v.name == param)
              .unwrap();
            match &to_edit.value {
              LuaComponentSettingValue::Boolean { value: _, default } => {
                to_edit.value = LuaComponentSettingValue::Boolean {
                  value,
                  default: *default,
                };
              }
              _ => panic!("invalid value"),
            };
            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::ModifyParameterString(path, param, value) => {
          if let Some(lcontent) = &mut context.layout.content {
            let component = get_mut_component_at_path(lcontent, path).unwrap();
            let parameters = component.get_parameters_mut();
            let to_edit = parameters
              .values
              .iter_mut()
              .find(|v| v.name == param)
              .unwrap();
            match &to_edit.value {
              LuaComponentSettingValue::String { value: _, default } => {
                to_edit.value = LuaComponentSettingValue::String {
                  value,
                  default: default.clone(),
                };
              }
              _ => panic!("invalid value"),
            };
            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::ModifyParameterOptions(path, param, value) => {
          if let Some(lcontent) = &mut context.layout.content {
            let component = get_mut_component_at_path(lcontent, path).unwrap();
            let parameters = component.get_parameters_mut();
            let to_edit = parameters
              .values
              .iter_mut()
              .find(|v| v.name == param)
              .unwrap();
            match &to_edit.value {
              LuaComponentSettingValue::Options {
                value: _,
                default,
                options: _,
              } => {
                to_edit.value = LuaComponentSettingValue::Options {
                  value,
                  default: default.clone(),
                  options: match &to_edit.value {
                    LuaComponentSettingValue::Options { options, .. } => options.clone(),
                    _ => panic!("invalid value"),
                  },
                };
              }
              _ => panic!("invalid value"),
            };
            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::ModifyParameterNumber(path, param, value) => {
          if let Some(lcontent) = &mut context.layout.content {
            if let Ok(value_f64) = value.parse::<f64>() {
              let component = get_mut_component_at_path(lcontent, path).unwrap();
              let parameters = component.get_parameters_mut();
              let to_edit = parameters
                .values
                .iter_mut()
                .find(|v| v.name == param)
                .unwrap();
              match &to_edit.value {
                LuaComponentSettingValue::Number { value: _, default } => {
                  to_edit.value = LuaComponentSettingValue::Number {
                    value: value_f64,
                    default: *default,
                  };
                }
                _ => panic!("invalid value"),
              };
            }
            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::ModifyParameterNumberRange(path, param, value) => {
          if let Some(lcontent) = &mut context.layout.content {
            let component = get_mut_component_at_path(lcontent, path).unwrap();
            let parameters = component.get_parameters_mut();
            let to_edit = parameters
              .values
              .iter_mut()
              .find(|v| v.name == param)
              .unwrap();
            match &to_edit.value {
              LuaComponentSettingValue::NumberRange {
                value: _,
                default,
                min,
                max,
                step,
              } => {
                to_edit.value = LuaComponentSettingValue::NumberRange {
                  value,
                  default: *default,
                  min: *min,
                  max: *max,
                  step: *step,
                };
              }
              _ => panic!("invalid value"),
            };
            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::ModifyParameterColorOpen(param) => {
          self
            .parameter_options_color_picker_opened
            .insert(param, true);
          Task::none()
        }
        LayoutEditorMessage::ModifyParameterColorSubmit(path, param, value) => {
          if let Some(lcontent) = &mut context.layout.content {
            self
              .parameter_options_color_picker_opened
              .insert(param.clone(), false);
            let component = get_mut_component_at_path(lcontent, path).unwrap();
            let parameters = component.get_parameters_mut();
            let to_edit = parameters
              .values
              .iter_mut()
              .find(|v| v.name == param)
              .unwrap();
            match &to_edit.value {
              LuaComponentSettingValue::Color { value: _, default } => {
                to_edit.value = LuaComponentSettingValue::Color {
                  value: [value.r, value.g, value.b, value.a],
                  default: *default,
                };
              }
              _ => panic!("invalid value"),
            };
            Task::none()
          } else {
            unreachable!()
          }
        }
        LayoutEditorMessage::ModifyParameterColorCancel(param) => {
          self
            .parameter_options_color_picker_opened
            .insert(param, false);
          Task::none()
        }
        LayoutEditorMessage::ModifyParameterImageOpen(path, param) => Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("Image Formats", &["png", "jpg", "jpeg"])
            .pick_file(),
        )
        .then(move |handle| match handle {
          Some(file_handle) => {
            let file_path = file_handle.path();
            match fs::read(file_path) {
              Ok(bytes) => Task::done(AppMessage::LayoutEditor(
                LayoutEditorMessage::ModifyParameterImageSubmit(path.clone(), param.clone(), bytes),
              )),
              Err(_) => Task::none(),
            }
          }
          None => Task::none(),
        }),
        LayoutEditorMessage::ModifyParameterImageSubmit(path, param, bytes) => {
          if let Some(lcontent) = &mut context.layout.content {
            let component = get_mut_component_at_path(lcontent, path).unwrap();
            let parameters = component.get_parameters_mut();
            let to_edit = parameters
              .values
              .iter_mut()
              .find(|v| v.name == param)
              .unwrap();
            match &to_edit.value {
              LuaComponentSettingValue::Image {
                bytes: _,
                handle: _,
              } => {
                to_edit.value = LuaComponentSettingValue::Image {
                  bytes: Some(bytes.clone()),
                  handle: Some(crate::lua::widgets::image::ImageHandleLua(
                    image::Handle::from_bytes(bytes),
                  )),
                };
              }
              _ => panic!("invalid value"),
            };
            Task::none()
          } else {
            unreachable!()
          }
        }
      },
      _ => Task::none(),
    }
  }

  fn view(&self, context: &AppContext) -> Element<'_, AppMessage> {
    let mut main_column_vec = Vec::new();

    main_column_vec.push(
      row(vec![
        text_input("Layout Name", &context.layout.name)
          .on_input(|i| AppMessage::LayoutEditor(LayoutEditorMessage::LayoutNameChanged(i)))
          .into(),
        text_input("Layout Author", &context.layout.author)
          .on_input(|i| AppMessage::LayoutEditor(LayoutEditorMessage::LayoutAuthorChanged(i)))
          .into(),
      ])
      .padding(5.0)
      .into(),
    );

    if let Some(lcontent) = &context.layout.content {
      main_column_vec.push(
        row(vec![
          column(build_tree_from_layout_part(
            lcontent,
            vec![],
            &self.opened_component,
          ))
          .width(Length::FillPortion(1))
          .padding(5.0)
          .into(),
          component_editor(
            &self,
            lcontent,
            self.opened_component.clone(),
            self.opened_component.clone(),
          )
          .unwrap(),
        ])
        .height(Length::Fill)
        .into(),
      );
    } else {
      main_column_vec.push(
        column(vec![
          text("You have no base component, please select one").into(),
          row(vec![
            combo_box(
              &self.new_component_combo_box_state,
              "Parts",
              self.new_component_combo_box_selected.as_ref(),
              |f| AppMessage::LayoutEditor(LayoutEditorMessage::NewComponentComboBoxSelected(f)),
            )
            .into(),
            button("Add Part")
              .on_press_maybe(self.new_component_combo_box_selected.as_ref().map(|f| {
                AppMessage::LayoutEditor(LayoutEditorMessage::AddNewComponent(vec![], f.clone()))
              }))
              .into(),
          ])
          .into(),
        ])
        .height(Length::Fill)
        .padding(5.0)
        .into(),
      );
    }

    column(main_column_vec).height(Length::Fill).into()
  }
}
