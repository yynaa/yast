use anyhow::Result;
use handy_keys::{Hotkey, HotkeyManager, Key, Modifiers};
use iced_aw::ContextMenu;
use yast_core::{
  layout::{Layout, component::Component},
  lua::{LuaContext, inject::inject_values_in_lua},
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
  Run, Segment, Timer as LSTimer,
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

pub struct App {
  window_id: Option<window::Id>,
  hotkey_manager: HotkeyManager,
  components: HashMap<String, String>,
  lua_context: LuaContext,
  pub layout: Layout,
  pub timer: LSTimer,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  Init(Option<window::Id>),
  Update,

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
}

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    let hotkeys = HotkeyManager::new().unwrap();

    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let timer = LSTimer::new(run).unwrap();

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

        hotkey_manager: hotkeys,

        components,
        lua_context,

        layout: Layout::default(),

        timer,
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
        if let Some(_event) = self.hotkey_manager.try_recv() {}

        Task::none()
      }
      AppMessage::WindowResized((_id, size)) => {
        self.layout.width = size.width;
        self.layout.height = size.height;
        Task::none()
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
        self.timer = LSTimer::new(parsed_run.run).unwrap();
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
        save_timer(&self.timer, IoWrite(writer)).unwrap();
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
    }
  }

  fn view(&self) -> Element<'_, AppMessage> {
    inject_values_in_lua(&self.lua_context.lua, &self.timer).unwrap();

    let inner = if let Some(lcontent) = &self.layout.content {
      lcontent.build().unwrap()
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
    .run()
}

fn main() -> Result<()> {
  pretty_env_logger::init_timed();

  run_app()?;

  Ok(())
}
