use anyhow::Result;
use mlua::prelude::*;

pub mod inject;
pub mod settings;
pub mod widgets;

pub struct LuaAppContext {
  pub lua: Lua,
}

impl LuaAppContext {
  pub fn init() -> Result<Self> {
    let lua = Lua::new();

    let package: LuaTable = lua.globals().get("package")?;
    let current_path: String = package.get("path")?;
    package.set("path", format!("{0}", current_path))?;

    settings::component_settings(&lua)?;
    widgets::widgets(&lua)?;

    Ok(Self { lua })
  }
}
