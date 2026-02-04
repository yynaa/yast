use iced::{
  Background, Element, Length, Task, Theme,
  widget::{button, column, space, stack},
  window,
};
use iced_aw::ContextMenu;

use crate::{
  app::{AppContext, AppMessage, Window},
  lua::inject::inject_values_in_lua,
};

pub struct Timer {}

#[derive(Clone, Debug)]
pub enum TimerMessage {}

impl Timer {
  pub fn open_window() -> Task<window::Id> {
    window::open(window::Settings {
      // decorations: false,
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

  fn update(&mut self, _context: &mut AppContext, message: AppMessage) -> Task<AppMessage> {
    match message {
      _ => Task::none(),
    }
  }

  fn view(&self, context: &AppContext) -> Element<'_, AppMessage> {
    inject_values_in_lua(&context.lua_context.lua, context).unwrap();

    let inner = if let Some(lcontent) = &context.layout.content {
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
          .style(styler)
          .into(),
        button("save splits")
          .width(Length::Fill)
          .style(styler)
          .into(),
        space().width(Length::Fixed(10.0)).into(),
        button("load layout")
          .width(Length::Fill)
          .style(styler)
          .into(),
        button("save layout")
          .width(Length::Fill)
          .style(styler)
          .into(),
        button("layout editor (beta)")
          .width(Length::Fill)
          .on_press(AppMessage::RequestLayoutEditor)
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
      // button(space())
      //   .on_press(AppMessage::DragTimer)
      //   .width(Length::Fill)
      //   .height(Length::Fill)
      //   .into(),
      context,
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }
}
