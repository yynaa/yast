use anyhow::Result;
use iced::Element;
use mlua::prelude::*;

use crate::{
  app::AppMessage,
  layout::LayoutPart,
  lua::widgets::{
    column::{init_lua_widget_column, LuaWidgetColumn},
    container::{init_lua_widget_container, LuaWidgetContainer},
    image::{init_lua_widget_image, LuaWidgetImage},
    internal::init_internals,
    row::{init_lua_widget_row, LuaWidgetRow},
    stack::{init_lua_widget_stack, LuaWidgetStack},
    text::{init_lua_widget_text, LuaWidgetText},
  },
};

pub mod column;
pub mod container;
pub mod image;
pub mod internal;
pub mod row;
pub mod stack;
pub mod text;

#[derive(Clone)]
pub enum LuaWidget {
  InternalChild(usize),

  Column(LuaWidgetColumn),

  Row(LuaWidgetRow),

  Stack(LuaWidgetStack),

  Container(LuaWidgetContainer),

  Image(LuaWidgetImage),

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
      LuaWidget::Row(inner) => inner.build(tree),
      LuaWidget::Stack(inner) => inner.build(tree),
      LuaWidget::Container(inner) => inner.build(tree),
      LuaWidget::Image(inner) => inner.build(),
      LuaWidget::Text(inner) => inner.build(),
    }
  }
}

pub fn widgets(lua: &Lua) -> Result<()> {
  init_internals(lua)?;

  let widgets = lua.create_table()?;
  init_lua_widget_text(lua, &widgets)?;
  init_lua_widget_column(lua, &widgets)?;
  init_lua_widget_row(lua, &widgets)?;
  init_lua_widget_stack(lua, &widgets)?;
  init_lua_widget_container(lua, &widgets)?;
  init_lua_widget_image(lua, &widgets)?;
  lua.globals().set("widgets", widgets)?;

  Ok(())
}
