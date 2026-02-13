use anyhow::Result;
use handy_keys::{Hotkey, Key, KeyboardListener};
use include_dir::Dir;
use livesplit_core::{Run, Segment, Timer};
use strum::IntoEnumIterator;
use yast_core::{
  defaults::copy_default_components,
  layout::{HotkeyAction, Layout, component::Component, settings::SettingsValue},
  lua::{
    LuaContext,
    inject::inject_values_in_lua,
    settings::{SettingsFactoryEntryContent, SettingsFactoryValue},
  },
  repository::Repository,
};

#[macro_use]
extern crate log;

use crate::{
  editor::component_editor,
  tree::{build_tree_from_layout_part, get_mut_component_at_path},
};
use iced::{
  Background, Color, Element, Length, Padding, Subscription, Task, Theme,
  alignment::Vertical,
  time::every,
  widget::{button, column, combo_box, container, image, row, space, stack, text, text_input},
};
use std::{
  backtrace::BacktraceStatus,
  collections::HashMap,
  fs::{self, read_to_string},
  time::{Duration, SystemTime},
};

pub mod editor;
pub mod tree;

static PROTOTYPE_VERSION: &str = env!("PROTOTYPE_VERSION");

pub enum AppScreen {
  LayoutEditor,
  LayoutPreview,
  HotkeyEditor,
}

pub struct App {
  components: HashMap<String, String>,
  lua_context: LuaContext,
  pub layout: Layout,
  pub repository: Repository,

  pub dummy_timer: Timer,

  pub screen: AppScreen,

  pub opened_component: Vec<usize>,
  pub new_component_combo_box_state: combo_box::State<String>,
  pub new_component_combo_box_selected: Option<String>,
  pub parameter_options_combo_box_states: HashMap<String, combo_box::State<String>>,

  pub keyboard_listener: KeyboardListener,
  pub hotkey_recorder: Option<HotkeyAction>,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  Update,

  LoadLayoutOpenPicker,
  LoadLayout(String),
  SaveLayoutOpenPicker,
  SaveLayout(String),
  TogglePreview,
  ToggleHotkeyEditor,

  LayoutNameChanged(String),
  LayoutAuthorChanged(String),

  OpenComponent(Vec<usize>),

  NewComponentComboBoxSelected(String),
  AddNewComponent(Vec<usize>, String),
  DeleteComponent(Vec<usize>),

  MoveComponentUp(Vec<usize>),
  MoveComponentDown(Vec<usize>),

  ModifyParameterBoolean(Vec<usize>, String, bool),
  ModifyParameterString(Vec<usize>, String, String),
  ModifyParameterOptions(Vec<usize>, String, String),
  ModifyParameterNumber(Vec<usize>, String, String),
  ModifyParameterNumberRange(Vec<usize>, String, f64),
  ModifyParameterColor(Vec<usize>, String, usize, String),
  ModifyParameterImageOpen(Vec<usize>, String),
  ModifyParameterImageSubmit(Vec<usize>, String, Vec<u8>),

  StartRecordingHotkey(HotkeyAction),
  AssignHotkey(Hotkey),
}

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    let mut run = Run::new();
    run.set_game_name("Game");
    run.set_category_name("Category");
    for i in 1..=10 {
      run.push_segment(Segment::new(&format!("Segment {}", i)));
    }

    let timer = Timer::new(run).expect("couldn't create dummy timer");

    let mut repository = Repository::default();
    repository.splits_icon.append(&mut vec![None; 10]);

    let lua_context = LuaContext::init().expect("couldn't initialize lua context");

    let mut components_dir = dirs::data_dir().expect("couldn't get data directory");
    components_dir.push("yast/components");
    let components = Component::import_all_from_directory(
      &components_dir.to_string_lossy().to_string(),
      &lua_context.lua,
    )
    .expect("couldn't get components");

    let mut new_component_options = Vec::new();

    let mut components_names = components
      .keys()
      .map(|s| s.clone())
      .collect::<Vec<String>>();
    components_names.sort();

    new_component_options.append(&mut components_names);

    (
      Self {
        components,
        lua_context,
        layout: Layout::default(),
        repository,
        dummy_timer: timer,

        screen: AppScreen::LayoutEditor,
        opened_component: Vec::new(),
        new_component_combo_box_state: combo_box::State::new(new_component_options),
        new_component_combo_box_selected: None,
        parameter_options_combo_box_states: HashMap::new(),

        keyboard_listener: KeyboardListener::new().expect("couldn't start keyboard listener"),
        hotkey_recorder: None,
      },
      Task::none(),
    )
  }

  fn update_handler(&mut self, message: AppMessage) -> Task<AppMessage> {
    match &message {
      AppMessage::Update => {}
      anything => {
        trace!("action: {:?}", anything);
      }
    }

    self.update(message.clone()).unwrap_or_else(|err| {
      error!(
        "error occurred updating message {:?}: {}",
        message.clone(),
        err
      );
      if let BacktraceStatus::Captured = err.backtrace().status() {
        error!("{}", err.backtrace());
      }
      Task::none()
    })
  }

  fn update(&mut self, message: AppMessage) -> Result<Task<AppMessage>> {
    match message {
      AppMessage::Update => {
        if let Some(event) = self.keyboard_listener.try_recv() {
          if self.hotkey_recorder.is_some() {
            if !event.is_key_down {
              return Ok(Task::done(AppMessage::AssignHotkey(event.as_hotkey()?)));
            }
          }
        }

        Ok(Task::none())
      }
      AppMessage::LoadLayoutOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("YAST Layout", &["yasl"])
            .pick_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::LoadLayout(file_path))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      AppMessage::LoadLayout(path) => {
        let toml_string = read_to_string(path)?;
        let new_layout = Layout::load(
          &mut self.repository,
          &self.components,
          &self.lua_context.lua,
          toml_string,
        )?;
        self.layout = new_layout;
        info!(
          "loaded layout: {} by {}",
          self.layout.name, self.layout.author
        );
        Ok(Task::none())
      }
      AppMessage::SaveLayoutOpenPicker => {
        let future = Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("YAST Layout", &["yasl"])
            .save_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_string_lossy().to_string();
            Task::done(AppMessage::SaveLayout(file_path))
          }
          None => Task::none(),
        });
        Ok(future)
      }
      AppMessage::SaveLayout(path) => {
        self.layout.save(&path)?;
        info!("saved layout");
        Ok(Task::none())
      }
      AppMessage::TogglePreview => {
        self.screen = match self.screen {
          AppScreen::LayoutPreview => AppScreen::LayoutEditor,
          _ => AppScreen::LayoutPreview,
        };
        Ok(Task::none())
      }
      AppMessage::ToggleHotkeyEditor => {
        self.screen = match self.screen {
          AppScreen::HotkeyEditor => AppScreen::LayoutEditor,
          _ => AppScreen::HotkeyEditor,
        };
        Ok(Task::none())
      }
      AppMessage::LayoutNameChanged(n) => {
        self.layout.name = n;
        Ok(Task::none())
      }
      AppMessage::LayoutAuthorChanged(n) => {
        self.layout.author = n;
        Ok(Task::none())
      }
      AppMessage::OpenComponent(n) => {
        if let Some(lcontent) = &mut self.layout.content {
          self.opened_component = n.clone();
          self.new_component_combo_box_selected = None;
          self.parameter_options_combo_box_states.clear();

          let comp = get_mut_component_at_path(lcontent, n)?;
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

        Ok(Task::none())
      }
      AppMessage::NewComponentComboBoxSelected(n) => {
        self.new_component_combo_box_selected = Some(n);
        Ok(Task::none())
      }
      AppMessage::AddNewComponent(path, name) => {
        let new_component = Component::from_str(
          self
            .components
            .get(&name)
            .ok_or(anyhow::Error::msg("couldn't find component in factories"))?
            .clone(),
          &self.lua_context.lua,
        )?;
        let param_defaults = new_component.parameters.initialize_defaults();
        let path_child = if let Some(lcontent) = &mut self.layout.content {
          let parent = get_mut_component_at_path(lcontent, path.clone())?;
          let mut path = path.clone();
          path.push(parent.children.len());
          path
        } else {
          path.clone()
        };

        for (param_name, param_value) in &param_defaults {
          match &param_value {
            SettingsValue::Image(_) => {
              self
                .repository
                .layout_images
                .insert((path_child.clone(), param_name.clone()), None);
            }
            _ => {}
          }
        }
        self
          .layout
          .settings
          .insert(path_child.clone(), param_defaults);

        if let Some(lcontent) = &mut self.layout.content {
          let parent = get_mut_component_at_path(lcontent, path.clone())?;
          parent.children.push(new_component);

          Ok(Task::none())
        } else {
          self.layout.content = Some(new_component);
          Ok(Task::done(AppMessage::OpenComponent(vec![])))
        }
      }
      AppMessage::DeleteComponent(mut path) => {
        if let Some(lcontent) = &mut self.layout.content {
          if path.len() > 0 {
            self.layout.settings.remove(&path);
            let last_path_element = path.pop().unwrap();
            let parent = get_mut_component_at_path(lcontent, path.clone())?;
            parent.children.remove(last_path_element);
            self.opened_component.pop();
            Ok(Task::none())
          } else {
            self.layout.content = None;
            Ok(Task::none())
          }
        } else {
          unreachable!()
        }
      }
      AppMessage::MoveComponentUp(path) => {
        let new_pos = self.layout.component_move_up(path)?;
        Ok(Task::done(AppMessage::OpenComponent(new_pos)))
      }
      AppMessage::MoveComponentDown(path) => {
        let new_pos = self.layout.component_move_down(path)?;
        Ok(Task::done(AppMessage::OpenComponent(new_pos)))
      }
      AppMessage::ModifyParameterBoolean(path, param, value) => {
        let comp_settings = self
          .layout
          .settings
          .get_mut(&path)
          .ok_or(anyhow::Error::msg(
            "couldn't find component in layout from path",
          ))?;
        comp_settings.insert(param, SettingsValue::Boolean(value));
        Ok(Task::none())
      }
      AppMessage::ModifyParameterString(path, param, value) => {
        let comp_settings = self
          .layout
          .settings
          .get_mut(&path)
          .ok_or(anyhow::Error::msg(
            "couldn't find component in layout from path",
          ))?;
        comp_settings.insert(param, SettingsValue::String(value));
        Ok(Task::none())
      }
      AppMessage::ModifyParameterOptions(path, param, value) => {
        let comp_settings = self
          .layout
          .settings
          .get_mut(&path)
          .ok_or(anyhow::Error::msg(
            "couldn't find component in layout from path",
          ))?;
        comp_settings.insert(param, SettingsValue::Options(value));
        Ok(Task::none())
      }
      AppMessage::ModifyParameterNumber(path, param, value) => {
        let comp_settings = self
          .layout
          .settings
          .get_mut(&path)
          .ok_or(anyhow::Error::msg(
            "couldn't find component in layout from path",
          ))?;
        if let Ok(parsed) = value.parse::<f64>() {
          comp_settings.insert(param, SettingsValue::Number(parsed));
        }
        Ok(Task::none())
      }
      AppMessage::ModifyParameterNumberRange(path, param, value) => {
        let comp_settings = self
          .layout
          .settings
          .get_mut(&path)
          .ok_or(anyhow::Error::msg(
            "couldn't find component in layout from path",
          ))?;
        comp_settings.insert(param, SettingsValue::NumberRange(value));
        Ok(Task::none())
      }
      AppMessage::ModifyParameterColor(path, param, index, value) => {
        let comp_settings = self
          .layout
          .settings
          .get_mut(&path)
          .ok_or(anyhow::Error::msg(
            "couldn't find component in layout from path",
          ))?;
        if let Some(SettingsValue::Color(color)) = comp_settings.get_mut(&param) {
          if let Ok(parsed) = value.parse::<f32>() {
            if parsed <= 255. {
              let channel_index = index.min(3);
              color[channel_index] = parsed / 255.;
            }
          }
        }
        Ok(Task::none())
      }
      AppMessage::ModifyParameterImageOpen(path, param) => {
        let future = Task::future(
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
        });
        Ok(future)
      }
      AppMessage::ModifyParameterImageSubmit(path, param, bytes) => {
        let comp_settings = self
          .layout
          .settings
          .get_mut(&path)
          .ok_or(anyhow::Error::msg(
            "couldn't find component in layout from path",
          ))?;
        comp_settings.insert(param.clone(), SettingsValue::Image(Some(bytes.clone())));
        self
          .repository
          .layout_images
          .insert((path, param), Some(image::Handle::from_bytes(bytes)));
        Ok(Task::none())
      }
      AppMessage::StartRecordingHotkey(action) => {
        self.hotkey_recorder = Some(action);
        Ok(Task::none())
      }
      AppMessage::AssignHotkey(hotkey) => {
        let mut is_escape = false;
        if let Some(key) = hotkey.key {
          if key == Key::Escape {
            is_escape = true;
          }
        }

        if let Some(action) = &self.hotkey_recorder {
          if is_escape {
            self.layout.hotkeys.remove(action);
          } else {
            self.layout.hotkeys.insert(action.clone(), hotkey);
          }
        }

        self.hotkey_recorder = None;
        Ok(Task::none())
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

    main_column_vec.push(
      row(vec![
        button("Layout Preview")
          .width(Length::Fill)
          .on_press(AppMessage::TogglePreview)
          .into(),
        button("Hotkey Editor")
          .width(Length::Fill)
          .on_press(AppMessage::ToggleHotkeyEditor)
          .into(),
      ])
      .padding(5.0)
      .spacing(5.0)
      .into(),
    );

    match self.screen {
      AppScreen::LayoutPreview => {
        inject_values_in_lua(&self.lua_context.lua, &self.dummy_timer, &self.repository)
          .unwrap_or_else(|err| error!("couldn't inject values into lua: {}", err));

        let inner = if let Some(lcontent) = &self.layout.content {
          lcontent
            .build(
              &self.lua_context.lua,
              vec![],
              &self.layout.settings,
              &self.repository,
            )
            .unwrap_or_else(|err| {
              error!("couldn't build layout: {}", err);
              text("couldn't build layout, please check the logs for full details")
                .width(Length::Fill)
                .height(Length::Fill)
                .center()
                .into()
            })
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
      }
      AppScreen::LayoutEditor => {
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
              .unwrap_or_else(|err| {
                error!("couldn't build component editor: {}", err);
                text("couldn't build component editor, please check the logs for full details")
                  .width(Length::Fill)
                  .height(Length::Fill)
                  .center()
                  .into()
              }),
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
      AppScreen::HotkeyEditor => {
        for action in HotkeyAction::iter() {
          let mut row_vec = Vec::new();

          let action_name = format!("{:?}", action);

          row_vec.push(
            button(text(action_name))
              .on_press(AppMessage::StartRecordingHotkey(action.clone()))
              .into(),
          );

          if let Some(hotkey) = self.layout.hotkeys.get(&action.clone()) {
            let hotkey_string = hotkey.to_string();
            row_vec.push(text(hotkey_string).into())
          }

          if let Some(recorded_action) = &self.hotkey_recorder {
            if *recorded_action == action {
              row_vec.push(text("recording...").into());
            }
          }

          main_column_vec.push(
            row(row_vec)
              .align_y(Vertical::Center)
              .spacing(5.0)
              .padding(Padding::new(5.0))
              .into(),
          );
        }
      }
    };

    column(main_column_vec).height(Length::Fill).into()
  }

  fn title(&self) -> String {
    format!("YASLE prototype {}", PROTOTYPE_VERSION)
  }

  fn subscription(&self) -> Subscription<AppMessage> {
    Subscription::batch(vec![
      every(Duration::from_secs_f64(1.0 / 60.0)).map(|_| AppMessage::Update),
    ])
  }
}

pub fn run_app() -> iced::Result {
  info!("starting YASLE prototype {}", PROTOTYPE_VERSION);

  iced::application(App::new, App::update_handler, App::view)
    .title(App::title)
    .theme(Theme::Dark)
    .subscription(App::subscription)
    .run()
}

#[cfg(target_os = "macos")]
fn is_ready() -> Result<bool> {
  let acc = handy_keys::check_accessibility();
  if !acc {
    handy_keys::open_accessibility_settings()?;
  }
  acc
}

#[cfg(not(target_os = "macos"))]
fn is_ready() -> Result<bool> {
  Ok(true)
}

static DEFAULT_DIR: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/../../default");

fn main() -> Result<()> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Warn)
    .level_for("yasle", log::LevelFilter::Info)
    .level_for("yast-core", log::LevelFilter::Info)
    .format(move |out, message, record| {
      out.finish(format_args!(
        "[{} || {}] {} » {}",
        humantime::format_rfc3339_seconds(SystemTime::now()),
        record.level(),
        record.target(),
        message
      ))
    })
    .chain(std::io::stdout())
    .chain(fern::log_file("yasle.log")?)
    .apply()?;

  copy_default_components(&DEFAULT_DIR)?;

  if is_ready()? {
    run_app()?;
  }

  Ok(())
}
