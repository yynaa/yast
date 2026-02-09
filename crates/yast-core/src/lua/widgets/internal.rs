use mlua::prelude::*;

use crate::lua::widgets::LuaWidget;

pub(super) fn init_internals(lua: &Lua) -> LuaResult<()> {
  let table = lua.create_table()?;

  let unique = lua.create_function(|_, index: usize| Ok(LuaWidget::InternalChild(index - 1)))?;
  table.set("get", unique)?;

  lua.globals().set("children", table)?;
  Ok(())
}
