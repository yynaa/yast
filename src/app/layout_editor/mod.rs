use std::{
  cell::RefCell,
  rc::Rc,
  sync::{Arc, Mutex},
};

use iced::{
  Element, Length, Task,
  widget::{column, combo_box, row, text_input},
  window,
};

use crate::{
  app::{
    AppContext, AppMessage, Window,
    layout_editor::{
      component_editor::component_editor,
      tree::{build_tree_from_layout_part, get_mut_component_at_path},
    },
  },
  layout::{LayoutPartIdentifier, containers::column::LayoutColumn},
};

mod component_editor;
mod tree;

pub struct LayoutEditor {
  pub opened_component: Vec<usize>,

  pub new_component_combo_box_state: combo_box::State<LayoutPartIdentifier>,
  pub new_component_combo_box_selected: Option<LayoutPartIdentifier>,
}

#[derive(Clone, Debug)]
pub enum LayoutEditorMessage {
  LayoutNameChanged(String),
  LayoutAuthorChanged(String),
  OpenComponent(Vec<usize>),
  NewComponentComboBoxSelected(LayoutPartIdentifier),
  AddNewComponent(Vec<usize>, LayoutPartIdentifier),
  DeleteComponent(Vec<usize>),
}

impl LayoutEditor {
  pub fn open_window() -> Task<window::Id> {
    window::open(window::Settings {
      ..Default::default()
    })
    .1
  }

  pub fn new(context: &AppContext) -> Self {
    let mut new_component_options = Vec::new();

    new_component_options.push(LayoutPartIdentifier::Column);

    new_component_options.append(
      &mut context
        .components
        .keys()
        .map(|s| LayoutPartIdentifier::Component(s.clone()))
        .collect::<Vec<LayoutPartIdentifier>>(),
    );

    Self {
      opened_component: Vec::new(),
      new_component_combo_box_state: combo_box::State::new(new_component_options),
      new_component_combo_box_selected: None,
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
          self.new_component_combo_box_selected = None;
          Task::none()
        }
        LayoutEditorMessage::NewComponentComboBoxSelected(n) => {
          self.new_component_combo_box_selected = Some(n);
          Task::none()
        }
        LayoutEditorMessage::AddNewComponent(path, id) => {
          let parent = get_mut_component_at_path(&mut context.layout.content, path).unwrap();
          let parent_children = parent.get_children_mut().unwrap();
          match id {
            LayoutPartIdentifier::Column => {
              parent_children.push(Box::new(LayoutColumn::new(Vec::new())));
            }
            LayoutPartIdentifier::Component(name) => {
              parent_children.push(Box::new(context.components.get(&name).unwrap().clone()));
            }
          };
          Task::none()
        }
        LayoutEditorMessage::DeleteComponent(mut path) => {
          let last_path_element = path.pop().unwrap();
          let parent = get_mut_component_at_path(&mut context.layout.content, path).unwrap();
          parent.get_children_mut().unwrap().remove(last_path_element);
          self.opened_component.pop();
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
          .into(),
        text_input("Layout Author", &context.layout.author)
          .on_input(|i| AppMessage::LayoutEditor(LayoutEditorMessage::LayoutAuthorChanged(i)))
          .into(),
      ])
      .padding(5.0)
      .into(),
      // layout editor
      row(vec![
        column(build_tree_from_layout_part(
          &context.layout.content,
          vec![],
          &self.opened_component,
        ))
        .width(Length::FillPortion(1))
        .padding(5.0)
        .into(),
        component_editor(
          &self,
          &context.layout.content,
          self.opened_component.clone(),
          self.opened_component.clone(),
        )
        .unwrap(),
      ])
      .height(Length::Fill)
      .into(),
    ])
    .into()
  }
}
