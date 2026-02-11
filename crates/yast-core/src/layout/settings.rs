use anyhow::Result;
use mlua::prelude::*;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
  pub fn inner(&self, lua: &Lua) -> Result<LuaValue> {
    let r = match self {
      Self::Boolean(v) => LuaValue::Boolean(*v),
      Self::String(v) => LuaValue::String(lua.create_string(v.clone())?),
      Self::Options(v) => LuaValue::String(lua.create_string(v.clone())?),
      Self::Number(v) => LuaValue::Number(*v),
      Self::NumberRange(v) => LuaValue::Number(*v),
      Self::Color(v) => todo!(),
      Self::Image(v) => LuaValue::Nil,
    };
    Ok(r)
  }
}
