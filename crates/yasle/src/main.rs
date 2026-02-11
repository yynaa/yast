use anyhow::Result;
use livesplit_core::{Run, Segment, Timer};
use yast_core::{
  layout::{Layout, component::Component, settings::SettingsValue},
  lua::{
    LuaContext,
    inject::inject_values_in_lua,
    settings::{SettingsFactoryEntryContent, SettingsFactoryValue},
  },
  repository::Repository,
};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::{
  editor::component_editor,
  tree::{build_tree_from_layout_part, get_mut_component_at_path},
};
use iced::{
  Background, Color, Element, Length, Task, Theme,
  widget::{button, column, combo_box, container, image, row, space, stack, text, text_input},
};
use std::{
  collections::HashMap,
  fs::{self, read_to_string},
};

pub mod editor;
pub mod tree;

static PROTOTYPE_VERSION: &str = env!("PROTOTYPE_VERSION");

pub struct App {
  components: HashMap<String, String>,
  lua_context: LuaContext,
  pub layout: Layout,
  pub repository: Repository,

  pub dummy_timer: Timer,

  pub preview: bool,

  pub opened_component: Vec<usize>,
  pub new_component_combo_box_state: combo_box::State<String>,
  pub new_component_combo_box_selected: Option<String>,
  pub parameter_options_combo_box_states: HashMap<String, combo_box::State<String>>,
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
  ModifyParameterColor(Vec<usize>, String, usize, String),
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
        repository: Repository::default(),
        dummy_timer: timer,

        preview: false,
        opened_component: Vec::new(),
        new_component_combo_box_state: combo_box::State::new(new_component_options),
        new_component_combo_box_selected: None,
        parameter_options_combo_box_states: HashMap::new(),
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
        let new_layout = Layout::load(
          &mut self.repository,
          &self.components,
          &self.lua_context.lua,
          toml_string,
        )
        .unwrap();
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

          let comp = get_mut_component_at_path(lcontent, n).unwrap();
          for p in &comp.parameters.0 {
            if let SettingsFactoryEntryContent::Value(name, value) = &p.content {
              match value {
                SettingsFactoryValue::Options(options, _) => {
                  self
                    .parameter_options_combo_box_states
                    .insert(name.clone(), combo_box::State::new(options.clone()));
                }
                _ => {}
              }
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
        let new_component = Component::from_str(
          self.components.get(&name).unwrap().clone(),
          &self.lua_context.lua,
        )
        .unwrap();
        let param_defaults = new_component.parameters.initialize_defaults();
        for (param_name, param_value) in &param_defaults {
          match &param_value {
            SettingsValue::Image(_) => {
              self
                .repository
                .layout_images
                .insert((path.clone(), param_name.clone()), None);
            }
            _ => {}
          }
        }
        self.layout.settings.insert(path.clone(), param_defaults);

        if let Some(lcontent) = &mut self.layout.content {
          let parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
          parent.children.push(new_component);

          Task::none()
        } else {
          self.layout.content = Some(new_component);
          Task::done(AppMessage::OpenComponent(vec![]))
        }
      }
      AppMessage::DeleteComponent(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
          if path.len() > 0 {
            let last_path_element = path.pop().unwrap();
            let parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
            parent.children.remove(last_path_element);
            self.opened_component.pop();
            self.layout.settings.remove(&path);
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
          let settings = self.layout.settings.remove(&path).unwrap();
          let last_path_element = path.pop().unwrap();
          if last_path_element > 0 {
            let parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
            let to_move = parent.children.remove(last_path_element);
            parent.children.insert(last_path_element - 1, to_move);
            self.opened_component = path.clone();
            self.opened_component.push(last_path_element - 1);
            path.push(last_path_element - 1);
            self.layout.settings.insert(path, settings);
          }

          Task::none()
        } else {
          unreachable!()
        }
      }
      AppMessage::MoveComponentDown(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
          let settings = self.layout.settings.remove(&path).unwrap();
          let last_path_element = path.pop().unwrap();
          let parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
          if last_path_element < parent.children.len() - 1 {
            let to_move = parent.children.remove(last_path_element);
            parent.children.insert(last_path_element + 1, to_move);
            self.opened_component = path.clone();
            self.opened_component.push(last_path_element + 1);
            path.push(last_path_element + 1);
            self.layout.settings.insert(path, settings);
          }

          Task::none()
        } else {
          unreachable!()
        }
      }
      AppMessage::EnterAboveComponent(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
          let settings = self.layout.settings.remove(&path).unwrap();
          let last_path_element = path.pop().unwrap();
          if last_path_element > 0 {
            let parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
            let to_move = parent.children.remove(last_path_element);
            let new_parent = parent.children.get_mut(last_path_element - 1).unwrap();
            new_parent.children.push(to_move);
            self.opened_component = path.clone();
            self.opened_component.push(last_path_element - 1);
            self.opened_component.push(new_parent.children.len() - 1);
            path.push(last_path_element - 1);
            path.push(new_parent.children.len() - 1);
            self.layout.settings.insert(path, settings);
          }

          Task::none()
        } else {
          unreachable!()
        }
      }
      AppMessage::ExitParentComponent(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
          if path.len() > 1 {
            let settings = self.layout.settings.remove(&path).unwrap();
            let last_path_element = path.pop().unwrap();
            let second_last_path_element = path.pop().unwrap();
            let parent_parent = get_mut_component_at_path(lcontent, path.clone()).unwrap();
            let myself = parent_parent
              .children
              .get_mut(second_last_path_element)
              .unwrap()
              .children
              .remove(last_path_element);
            parent_parent
              .children
              .insert(second_last_path_element, myself.clone());
            self.opened_component = path.clone();
            self.opened_component.push(second_last_path_element);
            path.push(second_last_path_element);
            self.layout.settings.insert(path, settings);
          }

          Task::none()
        } else {
          unreachable!()
        }
      }
      AppMessage::ModifyParameterBoolean(path, param, value) => {
        let comp_settings = self.layout.settings.get_mut(&path).unwrap();
        comp_settings.insert(param, SettingsValue::Boolean(value));
        Task::none()
      }
      AppMessage::ModifyParameterString(path, param, value) => {
        let comp_settings = self.layout.settings.get_mut(&path).unwrap();
        comp_settings.insert(param, SettingsValue::String(value));
        Task::none()
      }
      AppMessage::ModifyParameterOptions(path, param, value) => {
        let comp_settings = self.layout.settings.get_mut(&path).unwrap();
        comp_settings.insert(param, SettingsValue::Options(value));
        Task::none()
      }
      AppMessage::ModifyParameterNumber(path, param, value) => {
        let comp_settings = self.layout.settings.get_mut(&path).unwrap();
        if let Ok(parsed) = value.parse::<f64>() {
          comp_settings.insert(param, SettingsValue::Number(parsed));
        }
        Task::none()
      }
      AppMessage::ModifyParameterNumberRange(path, param, value) => {
        let comp_settings = self.layout.settings.get_mut(&path).unwrap();
        comp_settings.insert(param, SettingsValue::NumberRange(value));
        Task::none()
      }
      AppMessage::ModifyParameterColor(path, param, index, value) => {
        let comp_settings = self.layout.settings.get_mut(&path).unwrap();
        if let Some(SettingsValue::Color(color)) = comp_settings.get_mut(&param) {
          if let Ok(parsed) = value.parse::<f32>() {
            if parsed <= 255. {
              let channel_index = index.min(3);
              color[channel_index] = parsed / 255.;
            }
          }
        }
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
        let comp_settings = self.layout.settings.get_mut(&path).unwrap();
        comp_settings.insert(param.clone(), SettingsValue::Image(Some(bytes.clone())));
        self
          .repository
          .layout_images
          .insert((path, param), Some(image::Handle::from_bytes(bytes)));
        Task::none()
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
        lcontent
          .build(
            &self.lua_context.lua,
            vec![],
            &self.layout.settings,
            &self.repository,
          )
          .unwrap()
      } else {
        space().width(Length::Fill).height(Length::Fill).into()
      };

      let inner_with_background = stack(vec![
        container(space().width(Length::Fill).height(Length::Fill))
          .style(|_| container::Style {
            background: Some(Background::Color(Color::BLACK)),
            ..Default::default()
          })
          .into(),
        inner,
      ])
      .into();

      main_column_vec.push(inner_with_background);
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
    format!("YASLE prototype {}", PROTOTYPE_VERSION)
  }
}

pub fn run_app() -> iced::Result {
  info!("starting YASLE prototype {}", PROTOTYPE_VERSION);

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
