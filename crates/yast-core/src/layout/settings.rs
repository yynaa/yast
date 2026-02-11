use anyhow::Result;
use mlua::prelude::*;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{lua::widgets::image::ImageHandleLua, repository::Repository};

pub type LayoutSettings = HashMap<Vec<usize>, HashMap<String, SettingsValue>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettingsValue {
  Boolean(bool),
  String(String),
  Options(String),
  Number(f64),
  NumberRange(f64),
  Color([f32; 4]),
  Image(Option<Vec<u8>>),
}

impl SettingsValue {
  pub fn inner(
    &self,
    lua: &Lua,
    repository: &Repository,
    path: Vec<usize>,
    name: String,
  ) -> Result<LuaValue> {
    let r = match self {
      Self::Boolean(v) => LuaValue::Boolean(*v),
      Self::String(v) => LuaValue::String(lua.create_string(v.clone())?),
      Self::Options(v) => LuaValue::String(lua.create_string(v.clone())?),
      Self::Number(v) => LuaValue::Number(*v),
      Self::NumberRange(v) => LuaValue::Number(*v),
      Self::Color(v) => {
        let table = lua.create_table()?;

        table.push(v[0])?;
        table.push(v[1])?;
        table.push(v[2])?;
        table.push(v[3])?;

        LuaValue::Table(table)
      }
      Self::Image(_) => {
        let handle = repository
          .layout_images
          .get(&(path, name))
          .ok_or(anyhow::Error::msg("no image handle found in repository"))?;

        match handle {
          Some(h) => LuaValue::UserData(lua.create_userdata(ImageHandleLua(h.clone()))?),
          None => LuaValue::Nil,
        }
      }
    };
    Ok(r)
  }
}
