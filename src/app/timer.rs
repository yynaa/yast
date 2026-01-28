use iced::{
  Element, Task,
  widget::{center, space},
  window,
};

use crate::app::{AppMessage, Window};

pub struct Timer {}

pub enum TimerMessage {}

impl Timer {
  pub fn open_window() -> Task<window::Id> {
    window::open(window::Settings {
      ..Default::default()
    })
    .1
  }

  pub fn new() -> Self {
    Self {}
  }
}

impl Window for Timer {
  fn title(&self) -> String {
    String::from("YAST")
  }

  fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
    match message {
      _ => Task::none(),
    }
  }

  fn view(&self) -> Element<'_, AppMessage> {
    center("hello world!").into()
  }
}
