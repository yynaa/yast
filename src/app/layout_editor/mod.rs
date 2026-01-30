use iced::{
  Element, Length, Task,
  widget::{column, row, text, text_input},
  window,
};

use crate::app::{
  AppContext, AppMessage, Window,
  layout_editor::{component_editor::component_editor, tree::build_tree_from_layout_part},
};

mod component_editor;
mod tree;

pub struct LayoutEditor {
  pub opened_component: Vec<usize>,
}

#[derive(Clone, Debug)]
pub enum LayoutEditorMessage {
  LayoutNameChanged(String),
  LayoutAuthorChanged(String),
  OpenComponent(Vec<usize>),
}

impl LayoutEditor {
  pub fn open_window() -> Task<window::Id> {
    window::open(window::Settings {
      ..Default::default()
    })
    .1
  }

  pub fn new() -> Self {
    Self {
      opened_component: Vec::new(),
    }
  }
}

impl Window for LayoutEditor {
  fn title(&self) -> String {
    String::from("YAST Layout Editor")
  }

  fn update(&mut self, context: &mut AppContext, message: AppMessage) -> Task<AppMessage> {
    match message {
      AppMessage::LayoutEditor(inner) => match inner {
        LayoutEditorMessage::LayoutNameChanged(n) => {
          context.layout.name = n;
          Task::none()
        }
        LayoutEditorMessage::LayoutAuthorChanged(n) => {
          context.layout.author = n;
          Task::none()
        }
        LayoutEditorMessage::OpenComponent(n) => {
          self.opened_component = n;
          Task::none()
        }
        _ => Task::none(),
      },
      _ => Task::none(),
    }
  }

  fn view(&self, context: &AppContext) -> Element<'_, AppMessage> {
    column(vec![
      // top layout info pane
      row(vec![
        text_input("Layout Name", &context.layout.name)
          .on_input(|i| AppMessage::LayoutEditor(LayoutEditorMessage::LayoutNameChanged(i)))
          .padding(2.0)
          .into(),
        text_input("Layout Author", &context.layout.author)
          .on_input(|i| AppMessage::LayoutEditor(LayoutEditorMessage::LayoutAuthorChanged(i)))
          .padding(2.0)
          .into(),
      ])
      .into(),
      // layout editor
      row(vec![
        column(build_tree_from_layout_part(&context.layout.content, vec![]))
          .width(Length::FillPortion(1))
          .padding(2.0)
          .into(),
        component_editor(&context.layout.content, self.opened_component.clone()).unwrap(),
      ])
      .height(Length::Fill)
      .into(),
    ])
    .into()
  }
}
