use std::{
  collections::HashMap,
  fs::{read_dir, read_to_string},
  path::Path,
};

use anyhow::Result;
use iced::Element;
use mlua::prelude::*;

use crate::{app::AppMessage, layout::LayoutPart, lua::widgets::LuaWidget};

#[derive(Debug)]
pub struct Component {
  name: String,
  author: String,
  code: String,
}

impl LayoutPart for Component {
  fn build<'a>(&self, lua: &Lua) -> Result<Element<'a, AppMessage>> {
    let e = lua
      .load(format!("{}\n\nreturn widget()", self.code))
      .eval::<LuaWidget>()
      .unwrap()
      .build();
    Ok(e)
  }
}

impl Component {
  pub fn from_str(s: String) -> Result<Self> {
    let mut split = s.split("\n").filter(|s| s.starts_with("---"));
    let name = split
      .next()
      .ok_or(anyhow::Error::msg("missing name manifest"))?
      .split_at(3)
      .1
      .trim()
      .to_string();
    let author = split
      .next()
      .ok_or(anyhow::Error::msg("missing author manifest"))?
      .split_at(3)
      .1
      .trim()
      .to_string();

    Ok(Self {
      name,
      author,
      code: s,
    })
  }

  pub fn import_all_from_directory(p: &str) -> Result<HashMap<String, Self>> {
    let path = Path::new(p);
    let mut components = HashMap::new();
    if path.is_dir() {
      for file in read_dir(path)? {
        let entry = file?.path();
        if entry.is_file() {
          let st = read_to_string(entry)?;
          let comp = Self::from_str(st)?;
          components.insert(comp.name.clone(), comp);
        }
      }
    }
    Ok(components)
  }
}
