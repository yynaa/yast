use anyhow::Result;
use mlua::prelude::*;

pub mod settings;
pub mod widgets;

pub struct LuaAppContext {
  pub lua: Lua,
}

impl LuaAppContext {
  pub fn init() -> Result<Self> {
    let lua = Lua::new();

    match settings::component_settings(&lua) {
      Ok(it) => it,
      Err(err) => return Err(anyhow::Error::msg(err.to_string())),
    };

    match widgets::widgets(&lua) {
      Ok(it) => it,
      Err(err) => return Err(anyhow::Error::msg(err.to_string())),
    };

    Ok(Self { lua })
  }
}
