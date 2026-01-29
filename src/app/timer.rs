use iced::{Element, Task, widget::center, window};

use crate::{
  app::{AppContext, AppMessage, Window},
  lua::widgets::text::LuaWidgetText,
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
    const LUA_CODE: &str = include_str!("../../components/test.lua");
    context
      .lua_context
      .lua
      .load(format!("{}\n\nreturn widget()", LUA_CODE))
      .eval::<LuaWidgetText>()
      .unwrap()
      .build()
      .into()
  }
}
