use iced::{
  Alignment, Element, Length, Task,
  widget::{button, column, combo_box, row, text, text_input},
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
  layout::{LayoutPart, component::Component},
};

mod component_editor;
mod tree;

pub struct LayoutEditor {
  pub opened_component: Vec<usize>,

  pub new_component_combo_box_state: combo_box::State<String>,
  pub new_component_combo_box_selected: Option<String>,
}

#[derive(Clone, Debug)]
pub enum LayoutEditorMessage {
  LayoutNameChanged(String),
  LayoutAuthorChanged(String),
  OpenComponent(Vec<usize>),
  NewComponentComboBoxSelected(String),
  AddNewComponent(Vec<usize>, String),
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

    new_component_options.append(
      &mut context
        .components
        .keys()
        .map(|s| s.clone())
        .collect::<Vec<String>>(),
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
        LayoutEditorMessage::AddNewComponent(path, name) => {
          if let Some(lcontent) = &mut context.layout.content {
            let parent = get_mut_component_at_path(lcontent, path).unwrap();
            let parent_children = parent.get_children_mut().unwrap();
            parent_children.push(Box::new(
              Component::from_str(
                context.components.get(&name).unwrap().clone(),
                &context.lua_context.lua,
              )
              .unwrap(),
            ));
            Task::none()
          } else {
            context.layout.content = Some(Box::new(
              Component::from_str(
                context.components.get(&name).unwrap().clone(),
                &context.lua_context.lua,
              )
              .unwrap(),
            ));
            Task::none()
          }
        }
        LayoutEditorMessage::DeleteComponent(mut path) => {
          if let Some(lcontent) = &mut context.layout.content {
            if path.len() > 0 {
              let last_path_element = path.pop().unwrap();
              let parent = get_mut_component_at_path(lcontent, path).unwrap();
              parent.get_children_mut().unwrap().remove(last_path_element);
              self.opened_component.pop();
              Task::none()
            } else {
              context.layout.content = None;
              Task::none()
            }
          } else {
            unreachable!()
          }
        }
        _ => Task::none(),
      },
      _ => Task::none(),
    }
  }

  fn view(&self, context: &AppContext) -> Element<'_, AppMessage> {
    let mut main_column_vec = Vec::new();

    main_column_vec.push(
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
    );

    if let Some(lcontent) = &context.layout.content {
      main_column_vec.push(
        row(vec![
          column(build_tree_from_layout_part(
            lcontent,
            vec![],
            &self.opened_component,
          ))
          .width(Length::FillPortion(1))
          .padding(5.0)
          .into(),
          component_editor(
            &self,
            lcontent,
            self.opened_component.clone(),
            self.opened_component.clone(),
          )
          .unwrap(),
        ])
        .height(Length::Fill)
        .into(),
      );
    } else {
      main_column_vec.push(
        column(vec![
          text("You have no base component, please select one").into(),
          row(vec![
            combo_box(
              &self.new_component_combo_box_state,
              "Parts",
              self.new_component_combo_box_selected.as_ref(),
              |f| AppMessage::LayoutEditor(LayoutEditorMessage::NewComponentComboBoxSelected(f)),
            )
            .into(),
            button("Add Part")
              .on_press_maybe(self.new_component_combo_box_selected.as_ref().map(|f| {
                AppMessage::LayoutEditor(LayoutEditorMessage::AddNewComponent(vec![], f.clone()))
              }))
              .into(),
          ])
          .into(),
        ])
        .height(Length::Fill)
        .padding(5.0)
        .into(),
      );
    }

    column(main_column_vec).height(Length::Fill).into()
  }
}
