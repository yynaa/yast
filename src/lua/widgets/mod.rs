use anyhow::Result;
use iced::Element;
use mlua::prelude::*;

use crate::{
  app::AppMessage,
  layout::LayoutPart,
  lua::widgets::{
    column::{LuaWidgetColumn, init_lua_widget_column},
    internal::init_internals,
    text::{LuaWidgetText, init_lua_widget_text},
  },
};

pub mod column;
pub mod internal;
pub mod text;

#[derive(Clone)]
pub enum LuaWidget {
  InternalChild(usize),

  Column(LuaWidgetColumn),

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
  pub fn build<'a>(self, tree: &Box<dyn LayoutPart>) -> Element<'a, AppMessage> {
    match self {
      LuaWidget::InternalChild(index) => {
        let child = tree
          .get_children()
          .ok_or(anyhow::Error::msg("invalid path (no children)"))
          .unwrap()
          .get(index)
          .ok_or(anyhow::Error::msg("invalid path (no such child at index)"))
          .unwrap();

        child.build().unwrap()
      }
      LuaWidget::Column(inner) => inner.build(tree),
      LuaWidget::Text(inner) => inner.build(),
    }
  }
}

pub fn widgets(lua: &Lua) -> Result<()> {
  init_internals(lua)?;

  let widgets = lua.create_table()?;
  init_lua_widget_text(lua, &widgets)?;
  init_lua_widget_column(lua, &widgets)?;
  lua.globals().set("widgets", widgets)?;

  Ok(())
}
