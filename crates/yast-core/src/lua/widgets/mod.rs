use anyhow::Result;
use iced::Element;
use mlua::prelude::*;

use crate::{
  layout::{component::Component, settings::LayoutSettings},
  lua::widgets::{
    column::{LuaWidgetColumn, init_lua_widget_column},
    container::{LuaWidgetContainer, init_lua_widget_container},
    image::{LuaWidgetImage, init_lua_widget_image},
    internal::init_internals,
    row::{LuaWidgetRow, init_lua_widget_row},
    space::{LuaWidgetSpace, init_lua_widget_space},
    stack::{LuaWidgetStack, init_lua_widget_stack},
    text::{LuaWidgetText, init_lua_widget_text},
  },
};

pub mod column;
pub mod container;
pub mod image;
pub mod internal;
pub mod row;
pub mod space;
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

  Space(LuaWidgetSpace),
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
  pub fn build<'a, M: 'a>(
    self,
    tree: &Component,
    lua: &Lua,
    path: Vec<usize>,
    layout_settings: &LayoutSettings,
  ) -> Element<'a, M> {
    match self {
      LuaWidget::InternalChild(index) => {
        let child = tree
          .children
          .get(index)
          .ok_or(anyhow::Error::msg("invalid path (no such child at index)"))
          .unwrap();

        let mut new_path = path.clone();
        new_path.push(index);

        child.build(lua, new_path, layout_settings).unwrap()
      }
      LuaWidget::Column(inner) => inner.build(tree, lua, path, layout_settings),
      LuaWidget::Row(inner) => inner.build(tree, lua, path, layout_settings),
      LuaWidget::Stack(inner) => inner.build(tree, lua, path, layout_settings),
      LuaWidget::Container(inner) => inner.build(tree, lua, path, layout_settings),
      LuaWidget::Image(inner) => inner.build(),
      LuaWidget::Text(inner) => inner.build(),
      LuaWidget::Space(inner) => inner.build(),
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
  init_lua_widget_space(lua, &widgets)?;
  lua.globals().set("widgets", widgets)?;

  Ok(())
}
