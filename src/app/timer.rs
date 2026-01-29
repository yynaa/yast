use iced::{Element, Length, Task, widget::center, window};

use crate::{
  app::{AppContext, AppMessage, Window},
  layout::{LayoutPart, containers::column::LayoutColumn},
  lua::inject::inject_values_in_lua,
};

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

  fn update(&mut self, context: &mut AppContext, message: AppMessage) -> Task<AppMessage> {
    match message {
      _ => Task::none(),
    }
  }

  fn view(&self, context: &AppContext) -> Element<'_, AppMessage> {
    inject_values_in_lua(&context.lua_context.lua, context).unwrap();

    context
      .components
      .get("Test Component")
      .unwrap()
      .build()
      .unwrap()
  }
}
