use anyhow::Result;
use handy_keys::{Hotkey, HotkeyId, HotkeyManager, HotkeyState, Key, Modifiers};
use iced_aw::ContextMenu;
use yast_core::{
  layout::{Layout, component::Component},
  lua::{LuaContext, inject::inject_values_in_lua},
  repository::Repository,
};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use iced::{
  Background, Color, Element, Length, Size, Subscription, Task, Theme,
  time::every,
  widget::{button, column, container, space, stack},
  window,
};
use livesplit_core::{
  Run, Segment, SharedTimer, Timer,
  auto_splitting::Runtime,
  run::{
    parser,
    saver::livesplit::{IoWrite, save_timer},
  },
};
use std::{
  collections::HashMap,
  fs::{self, File, read_to_string},
  io::BufWriter,
  path::Path,
  time::Duration,
};

static PROTOTYPE_VERSION: &str = env!("PROTOTYPE_VERSION");

pub enum HotkeyAction {
  StartOrSplitTimer,
  ResetTimer,
  PauseTimer,
}

pub struct App {
  window_id: Option<window::Id>,
  hotkey_manager: HotkeyManager,
  hotkeys: HashMap<HotkeyId, HotkeyAction>,
  components: HashMap<String, String>,
  lua_context: LuaContext,
  pub layout: Layout,
  repository: Repository,
  pub timer: SharedTimer,
  #[allow(unused)]
  autosplitter: Runtime,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  Init(Option<window::Id>),
  Update,

  WindowClosing(window::Id),
  WindowResized((window::Id, Size)),
  ResizeTimer(f32, f32),

  LoadSplitsOpenPicker,
  LoadSplits(String),
  SaveSplitsOpenPicker,
  SaveSplits(String),
  LoadLayoutOpenPicker,
  LoadLayout(String),
  SaveLayoutOpenPicker,
  SaveLayout(String),
  LoadAutosplitterOpenPicker,
  LoadAutosplitter(String),
}

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    let hotkey_manager = HotkeyManager::new().unwrap();
    let mut hotkeys = HashMap::new();

    hotkeys.insert(
      hotkey_manager
        .register(Hotkey::new(Modifiers::CTRL, Key::S).unwrap())
        .unwrap(),
      HotkeyAction::StartOrSplitTimer,
    );

    hotkeys.insert(
      hotkey_manager
        .register(Hotkey::new(Modifiers::CTRL, Key::R).unwrap())
        .unwrap(),
      HotkeyAction::ResetTimer,
    );

    hotkeys.insert(
      hotkey_manager
        .register(Hotkey::new(Modifiers::CTRL, Key::P).unwrap())
        .unwrap(),
      HotkeyAction::PauseTimer,
    );

    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let timer = Timer::new(run).unwrap().into_shared();

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

        components,
        lua_context,

        layout: Layout::default(),
        repository: Repository::default(),

        timer,
        autosplitter,
      },
      window::latest().map(AppMessage::Init),
    )
  }

  fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
    match message {
      AppMessage::Init(id) => {
        self.window_id = id;
        Task::none()
      }
      AppMessage::Update => {
        if let Some(event) = self.hotkey_manager.try_recv() {
          if let HotkeyState::Pressed = event.state {
            let mut timer = self.timer.write().unwrap();
            match self.hotkeys.get(&event.id).unwrap() {
              HotkeyAction::StartOrSplitTimer => {
                timer.split_or_start();
              }
              HotkeyAction::ResetTimer => {
                timer.reset(true);
              }
              HotkeyAction::PauseTimer => {
                timer.toggle_pause();
              }
            }
          }
        }

        Task::none()
      }
      AppMessage::WindowResized((_id, size)) => {
        self.layout.width = size.width;
        self.layout.height = size.height;
        Task::none()
      }
      AppMessage::WindowClosing(_id) => {
        info!("closing YAST");

        iced::exit()
      }
      AppMessage::ResizeTimer(w, h) => window::resize(self.window_id.unwrap(), Size::new(w, h)),
      AppMessage::LoadSplitsOpenPicker => Task::future(
        rfd::AsyncFileDialog::new()
          .add_filter("Compatible Splits", &["lss"])
          .pick_file(),
      )
      .then(|handle| match handle {
        Some(handle) => {
          let file_path = handle.path().to_str().unwrap().to_string();
          Task::done(AppMessage::LoadSplits(file_path))
        }
        None => Task::none(),
      }),
      AppMessage::LoadSplits(path) => {
        let p = Path::new(&path);
        let source = fs::read(p).unwrap();
        let parsed_run = parser::parse_and_fix(&source, Some(p)).unwrap();
        self.timer = Timer::new(parsed_run.run).unwrap().into_shared();
        self.autosplitter = Runtime::new(self.timer.clone());
        Task::none()
      }
      AppMessage::SaveSplitsOpenPicker => Task::future(
        rfd::AsyncFileDialog::new()
          .add_filter("LiveSplit Splits", &["lss"])
          .save_file(),
      )
      .then(|handle| match handle {
        Some(handle) => {
          let file_path = handle.path().to_str().unwrap().to_string();
          Task::done(AppMessage::SaveSplits(file_path))
        }
        None => Task::none(),
      }),
      AppMessage::SaveSplits(path) => {
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        {
          let timer = self.timer.read().unwrap();
          save_timer(&timer, IoWrite(writer)).unwrap();
        }
        Task::none()
      }
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
        let width = new_layout.width;
        let height = new_layout.height;
        self.layout = new_layout;
        Task::done(AppMessage::ResizeTimer(width, height))
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
      AppMessage::LoadAutosplitterOpenPicker => Task::future(
        rfd::AsyncFileDialog::new()
          .add_filter("LiveSplit Autosplitter", &["wasm"])
          .pick_file(),
      )
      .then(|handle| match handle {
        Some(handle) => {
          let file_path = handle.path().to_str().unwrap().to_string();
          Task::done(AppMessage::LoadAutosplitter(file_path))
        }
        None => Task::none(),
      }),
      AppMessage::LoadAutosplitter(path) => {
        let p = Path::new(&path).to_path_buf();
        self.autosplitter.load_script_blocking(p).unwrap();
        Task::none()
      }
    }
  }

  fn view(&self) -> Element<'_, AppMessage> {
    {
      let timer = self.timer.read().unwrap();
      inject_values_in_lua(&self.lua_context.lua, &timer).unwrap();
    }

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

    let styler = |t: &Theme, _: button::Status| button::Style {
      background: Some(Background::Color(t.palette().primary)),
      text_color: t.palette().text,
      ..Default::default()
    };

    let context = ContextMenu::new(inner, move || {
      column(vec![
        button("load splits")
          .width(Length::Fill)
          .on_press(AppMessage::LoadSplitsOpenPicker)
          .style(styler)
          .into(),
        button("save splits")
          .width(Length::Fill)
          .on_press(AppMessage::SaveSplitsOpenPicker)
          .style(styler)
          .into(),
        space().width(Length::Fixed(10.0)).into(),
        button("load layout")
          .width(Length::Fill)
          .on_press(AppMessage::LoadLayoutOpenPicker)
          .style(styler)
          .into(),
        button("save layout")
          .width(Length::Fill)
          .on_press(AppMessage::SaveLayoutOpenPicker)
          .style(styler)
          .into(),
        space().width(Length::Fixed(10.0)).into(),
        button("exit").width(Length::Fill).style(styler).into(),
      ])
      .width(Length::Fixed(200.))
      .spacing(2.0)
      .into()
    })
    .into();

    stack(vec![
      container(space().width(Length::Fill).height(Length::Fill))
        .style(|_| container::Style {
          background: Some(Background::Color(Color::BLACK)),
          ..Default::default()
        })
        .into(),
      context,
    ])
    .into()
  }

  fn title(&self) -> String {
    format!("YAST prototype {}", PROTOTYPE_VERSION)
  }

  fn subscription(&self) -> Subscription<AppMessage> {
    Subscription::batch(vec![
      window::resize_events().map(AppMessage::WindowResized),
      window::close_requests().map(AppMessage::WindowClosing),
      every(Duration::from_secs_f64(1.0 / 60.0)).map(|_| AppMessage::Update),
    ])
  }
}

pub fn run_app() -> iced::Result {
  info!("starting YAST prototype {}", PROTOTYPE_VERSION);

  iced::application(App::new, App::update, App::view)
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

fn main() -> Result<()> {
  pretty_env_logger::init_timed();

  if is_ready()? {
    run_app()?;
  }

  Ok(())
}
