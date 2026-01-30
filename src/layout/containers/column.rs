use anyhow::Result;
use iced::widget::Column;
use iced::{Element, Length};
use mlua::prelude::*;

use crate::app::AppMessage;
use crate::layout::{LayoutPart, containers::LayoutContainer};

pub struct LayoutColumn {
  inner: Vec<Box<dyn LayoutPart>>,
}

impl LayoutColumn {
  pub fn new(inner: Vec<Box<dyn LayoutPart>>) -> Self {
    Self { inner }
  }
}

impl LayoutPart for LayoutColumn {
  fn build<'a>(&self) -> Result<Element<'a, AppMessage>> {
    let contents = self
      .inner
      .iter()
      .map(|b| b.as_ref().build().unwrap())
      .collect::<Vec<Element<'a, AppMessage>>>();

    Ok(
      Column::from_vec(contents)
        .height(Length::Fill)
        .width(Length::Fill)
        .into(),
    )
  }

  fn get_name(&self) -> String {
    "Column".to_string()
  }

  fn get_author(&self) -> String {
    "YAST".to_string()
  }

  fn get_children(&self) -> Option<&Vec<Box<dyn LayoutPart>>> {
    Some(&self.inner)
  }
}

impl LayoutContainer for LayoutColumn {}
