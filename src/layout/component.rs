use std::{
  collections::HashMap,
  fs::{read_dir, read_to_string},
  path::Path,
};

use anyhow::Result;
use iced::Element;
use mlua::prelude::*;

use crate::{app::AppMessage, layout::LayoutPart, lua::widgets::LuaWidget};

#[derive(Clone)]
pub struct Component {
  name: String,
  author: String,
  widget: LuaFunction,

  children: Vec<Box<dyn LayoutPart>>,
}

impl LayoutPart for Component {
  fn build<'a>(&self) -> Result<Element<'a, AppMessage>> {
    let e = self
      .widget
      .call::<LuaWidget>(self.children.len())?
      .build(&(Box::new(self.clone()) as Box<dyn LayoutPart + 'static>));
    Ok(e)
  }

  fn get_name(&self) -> String {
    self.name.clone()
  }

  fn get_author(&self) -> String {
    self.author.clone()
  }

  fn get_children(&self) -> Option<&Vec<Box<dyn LayoutPart>>> {
    Some(&self.children)
  }

  fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn LayoutPart>>> {
    Some(&mut self.children)
  }
}

impl Component {
  pub fn from_str(s: String, lua: &Lua) -> Result<Self> {
    let t = lua.load(s).eval::<LuaTable>()?;

    let r = Self {
      name: t.get("name")?,
      author: t.get("author")?,
      widget: t.get("widget")?,

      children: Vec::new(),
    };

    Ok(r)
  }

  pub fn import_all_from_directory(p: &str, lua: &Lua) -> Result<HashMap<String, String>> {
    let path = Path::new(p);
    let mut components = HashMap::new();
    if path.is_dir() {
      for file in read_dir(path)? {
        let entry = file?.path();
        if entry.is_file() {
          let st = read_to_string(entry)?;
          let name = lua
            .load(st.clone())
            .eval::<LuaTable>()?
            .get::<String>("name")?;
          components.insert(name, st);
        }
      }
    }
    Ok(components)
  }
}
