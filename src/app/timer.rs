use iced::{
  Element, Length, Task,
  widget::{button, column},
  window,
};
use iced_aw::ContextMenu;

use crate::{
  app::{AppContext, AppMessage, Window},
  layout::LayoutPart,
  lua::inject::inject_values_in_lua,
};

pub struct Timer {}

#[derive(Clone, Debug)]
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

    let inner = context.layout.content.build().unwrap();

    ContextMenu::new(inner, || {
      column(vec![
        button("splits editor").width(Length::Fill).into(),
        button("layout editor")
          .width(Length::Fill)
          .on_press(AppMessage::RequestLayoutEditor)
          .into(),
      ])
      .width(Length::Fixed(200.))
      .into()
    })
    .into()
  }
}
