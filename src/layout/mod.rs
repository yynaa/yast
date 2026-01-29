use anyhow::Result;
use iced::Element;
use mlua::prelude::*;

use crate::app::AppMessage;

pub mod component;
pub mod containers;

pub trait LayoutPart {
  fn build<'a>(&self, lua: &Lua) -> Result<Element<'a, AppMessage>>;
}
