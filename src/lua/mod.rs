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
    let mut current_path: String = package.get("path")?;
    let data_dir = dirs::data_dir()
      .ok_or(anyhow::Error::msg("couldn't get data dir"))?
      .to_string_lossy()
      .to_string();
    current_path.push_str(&format!(
      ";{0}/components/?.lua;{0}/components/?/init.lua;{0}/lib/?.lua",
      data_dir
    ));
    package.set("path", format!("{0}", current_path))?;

    settings::component_settings(&lua)?;
    widgets::widgets(&lua)?;

    Ok(Self { lua })
  }
}
