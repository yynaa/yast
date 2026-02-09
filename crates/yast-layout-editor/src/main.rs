use anyhow::Result;
use livesplit_core::{Run, Segment, Timer};
use yast_core::{
  layout::{Layout, component::Component},
  lua::{
    LuaContext, inject::inject_values_in_lua, settings::LuaComponentSettingValue,
    widgets::image::ImageHandleLua,
  },
};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::{
  editor::component_editor,
  tree::{build_tree_from_layout_part, get_mut_component_at_path},
};
use iced::{
  Color, Element, Length, Task, Theme,
  widget::{button, column, combo_box, image, row, space, text, text_input},
};
use std::{
  collections::HashMap,
  fs::{self, read_to_string},
};

pub mod editor;
pub mod tree;

pub struct App {
  components: HashMap<String, String>,
  lua_context: LuaContext,
  pub layout: Layout,
  pub dummy_timer: Timer,

  pub preview: bool,

  pub opened_component: Vec<usize>,
  pub new_component_combo_box_state: combo_box::State<String>,
  pub new_component_combo_box_selected: Option<String>,
  pub parameter_options_combo_box_states: HashMap<String, combo_box::State<String>>,
  pub parameter_options_color_picker_opened: HashMap<String, bool>,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  LoadLayoutOpenPicker,
  LoadLayout(String),
  SaveLayoutOpenPicker,
  SaveLayout(String),
  TogglePreview,

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

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    let mut run = Run::new();
    run.set_game_name("Game");
    run.set_category_name("Category");
    for i in 1..=10 {
      run.push_segment(Segment::new(&format!("Segment {}", i)));
    }
    let timer = Timer::new(run).unwrap();

    let lua_context = LuaContext::init().expect("couldn't initialize lua context");

    let mut components_dir = dirs::data_dir().expect("couldn't get data directory");
    components_dir.push("yast/components");
    let components = Component::import_all_from_directory(
      &components_dir.to_string_lossy().to_string(),
      &lua_context.lua,
    )
    .expect("couldn't get components");

    let mut new_component_options = Vec::new();

    new_component_options.append(
      &mut components
        .keys()
        .map(|s| s.clone())
        .collect::<Vec<String>>(),
    );

    (
      Self {
        components,
        lua_context,
        layout: Layout::default(),
        dummy_timer: timer,

        preview: false,
        opened_component: Vec::new(),
        new_component_combo_box_state: combo_box::State::new(new_component_options),
        new_component_combo_box_selected: None,
        parameter_options_combo_box_states: HashMap::new(),
        parameter_options_color_picker_opened: HashMap::new(),
      },
      Task::none(),
    )
  }

  fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
    match message {
      AppMessage::LoadLayoutOpenPicker => Task::future(
        rfd::AsyncFileDialog::new()
          .add_filter("YAST Layout", &["yasl"])
          .pick_file(),
      )
      .then(|handle| match handle {
        Some(handle) => {
          let file_path = handle.path().to_str().unwrap().to_string();
          Task::done(AppMessage::LoadLayout(file_path))
        }
        None => Task::none(),
      }),
      AppMessage::LoadLayout(path) => {
        let toml_string = read_to_string(path).unwrap();
        let new_layout =
          Layout::load(&self.components, &self.lua_context.lua, toml_string).unwrap();
        self.layout = new_layout;
        Task::none()
      }
      AppMessage::SaveLayoutOpenPicker => Task::future(
        rfd::AsyncFileDialog::new()
          .add_filter("YAST Layout", &["yasl"])
          .save_file(),
      )
      .then(|handle| match handle {
        Some(handle) => {
          let file_path = handle.path().to_str().unwrap().to_string();
          Task::done(AppMessage::SaveLayout(file_path))
        }
        None => Task::none(),
      }),
      AppMessage::SaveLayout(path) => {
        self.layout.save(&path).unwrap();
        Task::none()
      }
      AppMessage::TogglePreview => {
        self.preview = !self.preview;
        Task::none()
      }
      AppMessage::LayoutNameChanged(n) => {
        self.layout.name = n;
        Task::none()
      }
      AppMessage::LayoutAuthorChanged(n) => {
        self.layout.author = n;
        Task::none()
      }
      AppMessage::OpenComponent(n) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::NewComponentComboBoxSelected(n) => {
        self.new_component_combo_box_selected = Some(n);
        Task::none()
      }
      AppMessage::AddNewComponent(path, name) => {
        if let Some(lcontent) = &mut self.layout.content {
          let parent = get_mut_component_at_path(lcontent, path).unwrap();
          let parent_children = parent.get_children_mut().unwrap();
          parent_children.push(
            Component::from_str(
              self.components.get(&name).unwrap().clone(),
              &self.lua_context.lua,
            )
            .unwrap(),
          );
          Task::none()
        } else {
          self.layout.content = Some(
            Component::from_str(
              self.components.get(&name).unwrap().clone(),
              &self.lua_context.lua,
            )
            .unwrap(),
          );
          Task::done(AppMessage::OpenComponent(vec![]))
        }
      }
      AppMessage::DeleteComponent(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
          if path.len() > 0 {
            let last_path_element = path.pop().unwrap();
            let parent = get_mut_component_at_path(lcontent, path).unwrap();
            parent.get_children_mut().unwrap().remove(last_path_element);
            self.opened_component.pop();
            Task::none()
          } else {
            self.layout.content = None;
            Task::none()
          }
        } else {
          unreachable!()
        }
      }
      AppMessage::MoveComponentUp(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::MoveComponentDown(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::EnterAboveComponent(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::ExitParentComponent(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::ModifyParameterBoolean(path, param, value) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::ModifyParameterString(path, param, value) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::ModifyParameterOptions(path, param, value) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::ModifyParameterNumber(path, param, value) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::ModifyParameterNumberRange(path, param, value) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::ModifyParameterColorOpen(param) => {
        self
          .parameter_options_color_picker_opened
          .insert(param, true);
        Task::none()
      }
      AppMessage::ModifyParameterColorSubmit(path, param, value) => {
        if let Some(lcontent) = &mut self.layout.content {
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
      AppMessage::ModifyParameterColorCancel(param) => {
        self
          .parameter_options_color_picker_opened
          .insert(param, false);
        Task::none()
      }
      AppMessage::ModifyParameterImageOpen(path, param) => Task::future(
        rfd::AsyncFileDialog::new()
          .add_filter("Image Formats", &["png", "jpg", "jpeg"])
          .pick_file(),
      )
      .then(move |handle| match handle {
        Some(file_handle) => {
          let file_path = file_handle.path();
          match fs::read(file_path) {
            Ok(bytes) => Task::done(AppMessage::ModifyParameterImageSubmit(
              path.clone(),
              param.clone(),
              bytes,
            )),
            Err(_) => Task::none(),
          }
        }
        None => Task::none(),
      }),
      AppMessage::ModifyParameterImageSubmit(path, param, bytes) => {
        if let Some(lcontent) = &mut self.layout.content {
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
                handle: Some(ImageHandleLua(image::Handle::from_bytes(bytes))),
              };
            }
            _ => panic!("invalid value"),
          };
          Task::none()
        } else {
          unreachable!()
        }
      }
    }
  }

  fn view(&self) -> Element<'_, AppMessage> {
    let mut main_column_vec = Vec::new();

    main_column_vec.push(
      row(vec![
        button("Load Layout")
          .width(Length::Fill)
          .on_press(AppMessage::LoadLayoutOpenPicker)
          .into(),
        button("Save Layout")
          .width(Length::Fill)
          .on_press(AppMessage::SaveLayoutOpenPicker)
          .into(),
        button("Preview")
          .width(Length::Fill)
          .on_press(AppMessage::TogglePreview)
          .into(),
        text_input("Layout Name", &self.layout.name)
          .on_input(|i| AppMessage::LayoutNameChanged(i))
          .into(),
        text_input("Layout Author", &self.layout.author)
          .on_input(|i| AppMessage::LayoutAuthorChanged(i))
          .into(),
      ])
      .padding(5.0)
      .spacing(5.0)
      .into(),
    );

    if self.preview {
      inject_values_in_lua(&self.lua_context.lua, &self.dummy_timer).unwrap();

      let inner = if let Some(lcontent) = &self.layout.content {
        lcontent.build().unwrap()
      } else {
        space().width(Length::Fill).height(Length::Fill).into()
      };

      main_column_vec.push(inner);
    } else {
      if let Some(lcontent) = &self.layout.content {
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
                |f| AppMessage::NewComponentComboBoxSelected(f),
              )
              .into(),
              button("Add Part")
                .on_press_maybe(
                  self
                    .new_component_combo_box_selected
                    .as_ref()
                    .map(|f| AppMessage::AddNewComponent(vec![], f.clone())),
                )
                .into(),
            ])
            .into(),
          ])
          .height(Length::Fill)
          .padding(5.0)
          .into(),
        );
      }
    }

    column(main_column_vec).height(Length::Fill).into()
  }

  fn title(&self) -> String {
    String::from("YAST Layout Editor")
  }
}

pub fn run_app() -> iced::Result {
  info!("starting YAST Layout Editor app");

  iced::application(App::new, App::update, App::view)
    .title(App::title)
    .theme(Theme::Dark)
    .run()
}

fn main() -> Result<()> {
  pretty_env_logger::init_timed();

  run_app()?;

  Ok(())
}
