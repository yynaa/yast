use std::fs::read_to_string;

use iced::{
  Background, Element, Length, Task, Theme,
  widget::{button, column, space, stack},
  window,
};
use iced_aw::ContextMenu;

use crate::{
  app::{AppContext, AppMessage, Window},
  layout::Layout,
  lua::inject::inject_values_in_lua,
};

pub struct Timer {}

#[derive(Clone, Debug)]
pub enum TimerMessage {
  LoadLayoutOpenPicker,
  LoadLayout(String),
  SaveLayoutOpenPicker,
  SaveLayout(String),
}

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

  fn update(&mut self, context: &mut AppContext, message: AppMessage) -> Task<AppMessage> {
    match message {
      AppMessage::Timer(message) => match message {
        TimerMessage::LoadLayoutOpenPicker => Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("YAST Layout", &["yasl"])
            .pick_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_str().unwrap().to_string();
            Task::done(AppMessage::Timer(TimerMessage::LoadLayout(file_path)))
          }
          None => Task::none(),
        }),
        TimerMessage::LoadLayout(path) => {
          let toml_string = read_to_string(path).unwrap();
          let new_layout =
            Layout::load(&context.components, &context.lua_context.lua, toml_string).unwrap();
          let width = new_layout.width;
          let height = new_layout.height;
          context.layout = new_layout;
          Task::done(AppMessage::ResizeTimer(width, height))
        }
        TimerMessage::SaveLayoutOpenPicker => Task::future(
          rfd::AsyncFileDialog::new()
            .add_filter("YAST Layout", &["yasl"])
            .save_file(),
        )
        .then(|handle| match handle {
          Some(handle) => {
            let file_path = handle.path().to_str().unwrap().to_string();
            Task::done(AppMessage::Timer(TimerMessage::SaveLayout(file_path)))
          }
          None => Task::none(),
        }),
        TimerMessage::SaveLayout(path) => {
          context.layout.save(&path).unwrap();
          Task::none()
        }
      },
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
          .on_press(AppMessage::Timer(TimerMessage::LoadLayoutOpenPicker))
          .style(styler)
          .into(),
        button("save layout")
          .width(Length::Fill)
          .on_press(AppMessage::Timer(TimerMessage::SaveLayoutOpenPicker))
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
