use std::{
  collections::{BTreeMap, HashMap},
  time::Duration,
};

use iced::{Element, Subscription, Task, time::every, widget::space, window};
use livesplit_core::{Run, Segment, Timer as LSTimer};

use crate::{
  app::timer::{Timer, TimerMessage},
  layout::component::Component,
  lua::LuaAppContext,
};

mod timer;

pub trait Window: Send + Sync {
  fn title(&self) -> String;
  fn update(&mut self, context: &mut AppContext, message: AppMessage) -> Task<AppMessage>;
  fn view(&self, context: &AppContext) -> Element<'_, AppMessage>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowType {
  Timer,
}

pub struct App {
  window_ids: BTreeMap<window::Id, WindowType>,
  windows: BTreeMap<WindowType, Box<dyn Window>>,

  context: AppContext,
}

pub struct AppContext {
  components: HashMap<String, Component>,
  lua_context: LuaAppContext,

  pub timer: LSTimer,
}

pub enum AppMessage {
  WindowClosed(window::Id),
  OpenTimer(window::Id),
  UpdateView,

  Timer(TimerMessage),
}

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let mut timer = LSTimer::new(run).unwrap();
    timer.start();

    let lua_context = LuaAppContext::init().expect("couldn't initialize lua context");

    (
      Self {
        window_ids: BTreeMap::new(),
        windows: BTreeMap::new(),

        context: AppContext {
          components: Component::import_all_from_directory("components/", &lua_context.lua)
            .expect("couldn't get components"),
          lua_context,

          timer,
        },
      },
      Timer::open_window().map(AppMessage::OpenTimer),
    )
  }

  fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
    match message {
      AppMessage::WindowClosed(id) => {
        if let Some(typ) = self.window_ids.remove(&id) {
          self.windows.remove(&typ);
        }

        if self.windows.is_empty() {
          iced::exit()
        } else {
          Task::none()
        }
      }
      AppMessage::OpenTimer(id) => {
        let timer = Timer::new();
        self.window_ids.insert(id, WindowType::Timer);
        self.windows.insert(WindowType::Timer, Box::new(timer));
        Task::none()
      }
      AppMessage::Timer(_) => {
        if let Some(inner) = self.windows.get_mut(&WindowType::Timer) {
          inner.update(&mut self.context, message)
        } else {
          Task::none()
        }
      }
      _ => Task::none(),
    }
  }

  fn view(&self, window_id: window::Id) -> Element<'_, AppMessage> {
    if let Some(window_type) = self.window_ids.get(&window_id) {
      if let Some(window) = self.windows.get(&window_type) {
        return window.view(&self.context);
      }
    }
    space().into()
  }

  fn title(&self, window_id: window::Id) -> String {
    if let Some(window_type) = self.window_ids.get(&window_id) {
      if let Some(window) = self.windows.get(&window_type) {
        return window.title();
      }
    }
    String::from("YAST")
  }

  fn subscription(&self) -> Subscription<AppMessage> {
    Subscription::batch(vec![
      window::close_events().map(AppMessage::WindowClosed),
      every(Duration::from_secs_f64(1.0 / 30.0)).map(|_| AppMessage::UpdateView),
    ])
  }
}

pub fn run_app() -> iced::Result {
  info!("starting YAST app");

  iced::daemon(App::new, App::update, App::view)
    .subscription(App::subscription)
    .title(App::title)
    .run()
}
