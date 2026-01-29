use iced::Element;
use mlua::prelude::*;

use crate::{
  app::AppMessage,
  lua::widgets::text::{LuaWidgetText, init_lua_widget_text},
};

pub mod text;

#[derive(Clone)]
pub enum LuaWidget {
  Text(LuaWidgetText),
}

impl FromLua for LuaWidget {
  fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
    match value {
      LuaValue::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
      _ => unreachable!(),
    }
  }
}

impl LuaUserData for LuaWidget {}

impl LuaWidget {
  pub fn build<'a>(self) -> Element<'a, AppMessage> {
    match self {
      LuaWidget::Text(inner) => inner.build(),
    }
  }
}

pub fn widgets(lua: &Lua) -> LuaResult<()> {
  let widgets = lua.create_table()?;

  init_lua_widget_text(lua, &widgets)?;

  lua.globals().set("widgets", widgets)?;
  Ok(())
}
