use anyhow::Result;
use dyn_clone::DynClone;
use iced::Element;

use crate::app::AppMessage;

pub mod component;

pub trait LayoutPart: DynClone {
  fn build<'a>(&self) -> Result<Element<'a, AppMessage>>;

  fn get_name(&self) -> String;
  fn get_author(&self) -> String;
  fn get_children(&self) -> Option<&Vec<Box<dyn LayoutPart>>>;
  fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn LayoutPart>>>;
}

dyn_clone::clone_trait_object!(LayoutPart);

// serializing this may involve something like https://github.com/dtolnay/typetag
pub struct Layout {
  pub name: String,
  pub author: String,
  pub content: Option<Box<dyn LayoutPart>>,
}

impl Default for Layout {
  fn default() -> Self {
    Self {
      name: String::from("untitled"),
      author: String::new(),
      content: None,
    }
  }
}
