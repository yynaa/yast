use std::{
  collections::HashMap,
  fs::{read_dir, read_to_string},
  path::Path,
};

use anyhow::Result;
use iced::Element;
use mlua::prelude::*;

use crate::{app::AppMessage, layout::LayoutPart, lua::widgets::LuaWidget};

#[derive(Debug, Clone)]
pub struct Component {
  name: String,
  author: String,
  widget: LuaFunction,
}

impl LayoutPart for Component {
  fn build<'a>(&self) -> Result<Element<'a, AppMessage>> {
    let e = self.widget.call::<LuaWidget>(())?.build();
    Ok(e)
  }
}

impl Component {
  pub fn from_str(s: String, lua: &Lua) -> Result<Self> {
    let t = lua.load(s).eval::<LuaTable>()?;

    let r = Self {
      name: t.get("name")?,
      author: t.get("author")?,
      widget: t.get("widget")?,
    };

    Ok(r)
  }

  pub fn import_all_from_directory(p: &str, lua: &Lua) -> Result<HashMap<String, Self>> {
    let path = Path::new(p);
    let mut components = HashMap::new();
    if path.is_dir() {
      for file in read_dir(path)? {
        let entry = file?.path();
        if entry.is_file() {
          let st = read_to_string(entry)?;
          let comp = Self::from_str(st, lua)?;
          components.insert(comp.name.clone(), comp);
        }
      }
    }
    Ok(components)
  }
}
