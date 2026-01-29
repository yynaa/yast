use mlua::prelude::*;

use crate::lua::widgets::text::init_lua_widget_text;

pub mod text;

pub trait LuaWidget {}

pub fn widgets(lua: &Lua) -> LuaResult<()> {
  let widgets = lua.create_table()?;

  init_lua_widget_text(lua, &widgets)?;

  lua.globals().set("widgets", widgets)?;
  Ok(())
}
