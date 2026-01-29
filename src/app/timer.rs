use iced::{Element, Length, Task, widget::center, window};

use crate::{
  app::{AppContext, AppMessage, Window},
  layout::{LayoutPart, containers::column::LayoutColumn},
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
    let mut v: Vec<Box<dyn LayoutPart>> = Vec::new();

    for _ in 0..5 {
      let i = context.components.get("Test Component").unwrap();
      v.push(Box::new(i.clone()));
    }

    center(
      LayoutColumn::new(v)
        .build(&context.lua_context.lua)
        .unwrap(),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }
}
