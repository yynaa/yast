use anyhow::Result;
use iced::Element;
use mlua::prelude::*;

use crate::{app::AppMessage, layout::containers::column::LayoutColumn};

pub mod component;
pub mod containers;

pub trait LayoutPart {
  fn build<'a>(&self) -> Result<Element<'a, AppMessage>>;

  fn get_name(&self) -> String;
  fn get_author(&self) -> String;
  fn get_children(&self) -> Option<&Vec<Box<dyn LayoutPart>>>;
}

// serializing this may involve something like https://github.com/dtolnay/typetag
pub struct Layout {
  pub name: String,
  pub author: String,
  pub content: Box<dyn LayoutPart>,
}

impl Default for Layout {
  fn default() -> Self {
    Self {
      name: String::from("untitled"),
      author: String::new(),
      content: Box::new(LayoutColumn::new(Vec::new())),
    }
  }
}
