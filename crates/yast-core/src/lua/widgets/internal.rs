//! injector for internal functions (children, logging)

use log::{debug, error, info, trace, warn};
use mlua::prelude::*;

use crate::lua::widgets::LuaWidget;

/// internals injector
pub(super) fn init_internals(lua: &Lua) -> LuaResult<()> {
  let children_table = lua.create_table()?;

  let unique = lua.create_function(|_, index: usize| Ok(LuaWidget::InternalChild(index - 1)))?;
  children_table.set("get", unique)?;

  lua.globals().set("children", children_table)?;

  let logging_table = lua.create_table()?;

  logging_table.set(
    "debug",
    lua.create_function(|_, msg: String| {
      debug!("lualog: {}", msg);
      Ok(())
    })?,
  )?;
  logging_table.set(
    "trace",
    lua.create_function(|_, msg: String| {
      trace!("lualog: {}", msg);
      Ok(())
    })?,
  )?;
  logging_table.set(
    "info",
    lua.create_function(|_, msg: String| {
      info!("lualog: {}", msg);
      Ok(())
    })?,
  )?;
  logging_table.set(
    "warn",
    lua.create_function(|_, msg: String| {
      warn!("lualog: {}", msg);
      Ok(())
    })?,
  )?;
  logging_table.set(
    "error",
    lua.create_function(|_, msg: String| {
      error!("lualog: {}", msg);
      Ok(())
    })?,
  )?;

  lua.globals().set("log", logging_table)?;

  Ok(())
}
