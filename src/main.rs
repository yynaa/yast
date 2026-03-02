use anyhow::Result;
use handy_keys::{HotkeyId, HotkeyManager};
use include_dir::Dir;
use yast_core::{
  defaults::copy_default_components,
  layout::{HotkeyAction, Layout, component::Component},
  lua::{LuaContext, inject::inject_values_in_lua},
  repository::Repository,
};

#[macro_use]
extern crate log;

#[cfg(target_os = "windows")]
use iced::keyboard;
use iced::{
  Background, Color, Element, Length, Size, Subscription, Task, Theme,
  time::every,
  widget::{container, mouse_area, space, stack, text},
  window,
};
use livesplit_core::{
  Run, Segment, SharedTimer, Timer,
  auto_splitting::Runtime,
  run::saver::livesplit::{IoWrite, save_timer},
};
use std::time::Duration;
use std::{collections::HashMap, fs::File, io::BufWriter, time::SystemTime};

use crate::menu::{Menu, MenuMessage};

mod menu;
mod update;

static VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct App {
  window_id: Option<window::Id>,
  hotkey_manager: HotkeyManager,
  hotkeys: HashMap<HotkeyId, HotkeyAction>,
  hotkeys_on: bool,
  components: HashMap<String, String>,
  lua_context: LuaContext,
  pub layout: Layout,
  repository: Repository,
  pub timer: SharedTimer,
  #[allow(unused)]
  autosplitter: Runtime,
  splits_edited: bool,
  menu: Menu,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  Init(Option<window::Id>),
  Update,

  WindowClosing(window::Id),
  WindowResized((window::Id, Size)),
  #[cfg(target_os = "windows")]
  KeyboardEvent(keyboard::Event),
  ResizeTimer(f32, f32),

  MenuMessage(MenuMessage),
}

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    let hotkey_manager = HotkeyManager::new().expect("couldn't initialize hotkeys");
    let hotkeys = HashMap::new();

    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let timer = Timer::new(run)
      .expect("couldn't initialize timer")
      .into_shared();
    let mut repository = Repository::default();
    repository.splits_icon.push(None);

    let autosplitter = Runtime::new(timer.clone());

    let lua_context = LuaContext::init().expect("couldn't initialize lua context");

    let mut components_dir = dirs::data_dir().expect("couldn't get data directory");
    components_dir.push("yast/components");
    let components = Component::import_all_from_directory(
      &components_dir.to_string_lossy().to_string(),
      &lua_context.lua,
    )
    .expect("couldn't get components");

    (
      Self {
        window_id: None,

        hotkey_manager,
        hotkeys,
        hotkeys_on: false,

        components,
        lua_context,

        layout: Layout::default(),
        repository,

        timer,
        autosplitter,

        splits_edited: false,

        menu: Menu::new(),
      },
      window::latest().map(AppMessage::Init),
    )
  }

  fn view(&self) -> Element<'_, AppMessage> {
    if let Ok(timer) = self.timer.read() {
      inject_values_in_lua(&self.lua_context.lua, &timer, &self.repository)
        .unwrap_or_else(|err| error!("couldn't inject values into lua: {}", err));
    }

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

    let mut stack_vec = vec![
      mouse_area(
        container(space().width(Length::Fill).height(Length::Fill)).style(|_| container::Style {
          background: Some(Background::Color(Color::BLACK)),
          ..Default::default()
        }),
      )
      .on_right_press(AppMessage::MenuMessage(MenuMessage::ToggleMenu))
      .into(),
      inner,
    ];

    if self.menu.opened {
      stack_vec.push(
        container(space().width(Length::Fill).height(Length::Fill))
          .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.5))),
            ..Default::default()
          })
          .into(),
      );
      stack_vec.push(Menu::view(&self));
    }

    let stacked = stack(stack_vec).into();

    stacked
  }

  fn title(&self) -> String {
    format!("YAST {}", VERSION)
  }

  #[cfg(not(target_os = "windows"))]
  fn subscription(&self) -> Subscription<AppMessage> {
    Subscription::batch(vec![
      window::resize_events().map(AppMessage::WindowResized),
      window::close_requests().map(AppMessage::WindowClosing),
      every(Duration::from_secs_f64(1.0 / 60.0)).map(|_| AppMessage::Update),
    ])
  }

  #[cfg(target_os = "windows")]
  fn subscription(&self) -> Subscription<AppMessage> {
    Subscription::batch(vec![
      window::resize_events().map(AppMessage::WindowResized),
      window::close_requests().map(AppMessage::WindowClosing),
      keyboard::listen().map(AppMessage::KeyboardEvent),
      every(Duration::from_secs_f64(1.0 / 60.0)).map(|_| AppMessage::Update),
    ])
  }

  fn save_splits(&self, path: String) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    {
      let timer = self
        .timer
        .read()
        .map_err(|_| anyhow::Error::msg("couldn't access timer"))?;
      save_timer(&timer, IoWrite(writer))?;
    }
    Ok(())
  }
}

pub fn run_app() -> iced::Result {
  info!("starting YAST {}", VERSION);

  iced::application(App::new, App::update_handler, App::view)
    .subscription(App::subscription)
    .title(App::title)
    .theme(Theme::Dark)
    .exit_on_close_request(false)
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

static DEFAULT_DIR: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/default");

fn main() -> Result<()> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Warn)
    .level_for("yast", log::LevelFilter::Info)
    .level_for("yast_core", log::LevelFilter::Info)
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
    .chain(fern::log_file("yast.log")?)
    .apply()?;

  copy_default_components(&DEFAULT_DIR)?;

  if is_ready()? {
    run_app()?;
  }

  Ok(())
}
