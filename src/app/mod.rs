use std::collections::BTreeMap;

use iced::{Element, Subscription, Task, widget::space, window};

use crate::app::timer::{Timer, TimerMessage};

mod timer;

pub trait Window: Send + Sync {
  fn title(&self) -> String;
  fn update(&mut self, message: AppMessage) -> Task<AppMessage>;
  fn view(&self) -> Element<'_, AppMessage>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowType {
  Timer,
}

pub struct App {
  window_ids: BTreeMap<window::Id, WindowType>,
  windows: BTreeMap<WindowType, Box<dyn Window>>,
}

pub enum AppMessage {
  WindowClosed(window::Id),
  OpenTimer(window::Id),

  Timer(TimerMessage),
}

impl App {
  fn new() -> (Self, Task<AppMessage>) {
    (
      Self {
        window_ids: BTreeMap::new(),
        windows: BTreeMap::new(),
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
          inner.update(message)
        } else {
          Task::none()
        }
      }
    }
  }

  fn view(&self, window_id: window::Id) -> Element<'_, AppMessage> {
    if let Some(window_type) = self.window_ids.get(&window_id) {
      if let Some(window) = self.windows.get(&window_type) {
        return window.view().into();
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
    window::close_events().map(AppMessage::WindowClosed)
  }
}

pub fn run_app() -> iced::Result {
  iced::daemon(App::new, App::update, App::view)
    .subscription(App::subscription)
    .title(App::title)
    .run()
}
