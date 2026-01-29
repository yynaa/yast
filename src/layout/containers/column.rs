use anyhow::Result;
use iced::Element;
use iced::widget::Column;
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
  fn build<'a>(&self, lua: &Lua) -> Result<Element<'a, AppMessage>> {
    let contents = self
      .inner
      .iter()
      .map(|b| b.as_ref().build(lua).unwrap())
      .collect::<Vec<Element<'a, AppMessage>>>();

    Ok(Column::from_vec(contents).into())
  }
}

impl LayoutContainer for LayoutColumn {}
